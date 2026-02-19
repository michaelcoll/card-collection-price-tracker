use crate::application::error::AppError;
use crate::application::repository::CardCollectionRepository;
use crate::domain::user::User;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

pub struct CardCollectionRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardCollectionRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardCollectionRepository for CardCollectionRepositoryAdapter {
    async fn get_date_and_user_to_update(&self) -> Result<Vec<(NaiveDate, User)>, AppError> {
        let rows = sqlx::query!(
            r#"
                SELECT dates.date, users.user_id
                FROM (SELECT DISTINCT user_id FROM card_quantity) AS users
                         CROSS JOIN (SELECT DISTINCT date FROM cardmarket_price) AS dates
                WHERE (dates.date, users.user_id) NOT IN (SELECT date, user_id
                                                            FROM collection_price_history)
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let result = rows
            .into_iter()
            .map(|row| (row.date, User::from_id(row.user_id)))
            .collect();

        Ok(result)
    }

    async fn update_for_date_and_user(&self, date: NaiveDate, user: User) -> Result<(), AppError> {
        sqlx::query!(
            r#"
                INSERT INTO collection_price_history
                SELECT prices.date,
                       prices.user_id,
                       SUM(prices.low)   AS low,
                       SUM(prices.avg)   AS avg,
                       SUM(prices.trend) AS trend,
                       SUM(prices.avg1)  AS avg1,
                       SUM(prices.avg7)  AS avg7,
                       SUM(prices.avg30) AS avg30

                FROM (SELECT cq.user_id,
                             cmp.date,
                             CASE WHEN c.foil THEN cmp.low_foil ELSE cmp.low END * cq.quantity     AS low,
                             CASE WHEN c.foil THEN cmp.avg_foil ELSE cmp.avg END * cq.quantity     AS avg,
                             CASE WHEN c.foil THEN cmp.trend_foil ELSE cmp.trend END * cq.quantity AS trend,
                             CASE WHEN c.foil THEN cmp.avg1_foil ELSE cmp.avg1 END * cq.quantity   AS avg1,
                             CASE WHEN c.foil THEN cmp.avg7_foil ELSE cmp.avg7 END * cq.quantity   AS avg7,
                             CASE WHEN c.foil THEN cmp.avg30_foil ELSE cmp.avg30 END * cq.quantity AS avg30
                      FROM card c
                               JOIN card_quantity cq
                                    ON c.set_code = cq.set_code AND c.collector_number = cq.collector_number AND
                                       c.language_code = cq.language_code AND c.foil = cq.foil
                               JOIN cardmarket_price cmp ON cardmarket_id = cmp.id_produit) AS prices
                WHERE prices.user_id = $1
                  AND prices.date = $2
                GROUP BY prices.user_id, prices.date
                "#,
            user.id,
            date,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn get_date_and_user_to_update_returns_empty_when_no_data(pool: PgPool) {
        let adapter = CardCollectionRepositoryAdapter::new(pool);
        let result = adapter.get_date_and_user_to_update().await.unwrap();

        assert!(result.is_empty());
    }

    #[sqlx::test]
    async fn get_date_and_user_to_update_returns_combinations_not_in_history(pool: PgPool) {
        let adapter = CardCollectionRepositoryAdapter::new(pool.clone());

        sqlx::query!(
            r#"
                INSERT INTO set_name (set_code, name)
                VALUES ('SET1', 'Test Set 1')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card (set_code, collector_number, language_code, foil, name, scryfall_id, cardmarket_id)
                VALUES ('SET1', '1', 'EN', false, 'Test Card', $1, 1)
            "#,
            uuid::Uuid::nil()
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
                VALUES ('SET1', '1', 'EN', false, 'user1', 1, 100)
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
                VALUES (1, $1, 10, 20, 15, 25, 18, 22)
            "#,
            NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = adapter.get_date_and_user_to_update().await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
        assert_eq!(result[0].1.id, "user1");
    }

    #[sqlx::test]
    async fn get_date_and_user_to_update_excludes_combinations_in_history(pool: PgPool) {
        let adapter = CardCollectionRepositoryAdapter::new(pool.clone());

        sqlx::query!(
            r#"
                INSERT INTO set_name (set_code, name)
                VALUES ('SET2', 'Test Set 2')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card (set_code, collector_number, language_code, foil, name, scryfall_id, cardmarket_id)
                VALUES ('SET2', '1', 'EN', false, 'Test Card', $1, 1)
            "#,
            uuid::Uuid::nil()
        )
        .execute(&pool)
        .await
        .unwrap();

        let date = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        sqlx::query!(
            r#"
                INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
                VALUES ('SET2', '1', 'EN', false, 'user1', 1, 100)
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
                VALUES (1, $1, 10, 20, 15, 25, 18, 22)
            "#,
            date
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO collection_price_history (date, user_id, low, avg, trend, avg1, avg7, avg30)
                VALUES ($1, 'user1', 100, 200, 150, 250, 180, 220)
            "#,
            date
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = adapter.get_date_and_user_to_update().await.unwrap();

        assert!(result.is_empty());
    }

    #[sqlx::test]
    async fn get_date_and_user_to_update_returns_multiple_combinations(pool: PgPool) {
        let adapter = CardCollectionRepositoryAdapter::new(pool.clone());

        sqlx::query!(
            r#"
                INSERT INTO set_name (set_code, name)
                VALUES ('SET3', 'Test Set 3')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card (set_code, collector_number, language_code, foil, name, scryfall_id, cardmarket_id)
                VALUES ('SET3', '1', 'EN', false, 'Test Card', $1, 1)
            "#,
            uuid::Uuid::nil()
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
                VALUES ('SET3', '1', 'EN', false, 'user1', 1, 100)
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
                VALUES ('SET3', '1', 'EN', false, 'user2', 2, 200)
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let date1 = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2025, 12, 26).unwrap();

        sqlx::query!(
            r#"
                INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
                VALUES (1, $1, 10, 20, 15, 25, 18, 22)
            "#,
            date1
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
                VALUES (1, $1, 12, 22, 17, 27, 20, 24)
            "#,
            date2
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = adapter.get_date_and_user_to_update().await.unwrap();

        assert_eq!(result.len(), 4);
    }

    #[sqlx::test]
    async fn update_for_date_and_user_filters_by_user(pool: PgPool) {
        let adapter = CardCollectionRepositoryAdapter::new(pool.clone());
        let user1 = User::from_id("user1".to_string());
        let date = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();

        sqlx::query!(
            r#"
                INSERT INTO set_name (set_code, name)
                VALUES ('SET6', 'Test Set 6')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card (set_code, collector_number, language_code, foil, name, scryfall_id, cardmarket_id)
                VALUES ('SET6', '1', 'EN', false, 'Card 1', $1, 5)
            "#,
            uuid::Uuid::nil()
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
                VALUES ('SET6', '1', 'EN', false, 'user1', 2, 100),
                       ('SET6', '1', 'EN', false, 'user2', 3, 150)
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"
                INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
                VALUES (5, $1, 10, 20, 15, 25, 18, 22)
            "#,
            date
        )
        .execute(&pool)
        .await
        .unwrap();

        adapter
            .update_for_date_and_user(date, user1.clone())
            .await
            .unwrap();

        let rows_user1 = sqlx::query!(
            r#"
                SELECT user_id, low
                FROM collection_price_history
                WHERE date = $1 AND user_id = 'user1'
            "#,
            date
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        let rows_user2 = sqlx::query!(
            r#"
                SELECT user_id, low
                FROM collection_price_history
                WHERE date = $1 AND user_id = 'user2'
            "#,
            date
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        assert_eq!(rows_user1.len(), 1);
        assert_eq!(rows_user1[0].low, 20i32); // 10 * 2
        assert_eq!(rows_user2.len(), 0);
    }
}
