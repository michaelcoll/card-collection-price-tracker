use crate::application::error::AppError;
use crate::application::repository::CollectionPriceHistoryRepository;
use crate::domain::price::{PriceGuide, PriceHistoryEntry};
use crate::domain::user::User;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

pub struct CollectionPriceHistoryRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CollectionPriceHistoryRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionPriceHistoryRepository for CollectionPriceHistoryRepositoryAdapter {
    async fn get_date_and_user_to_update(&self) -> Result<Vec<(NaiveDate, User)>, AppError> {
        let rows = sqlx::query!(
            r#"
                SELECT dates.date, users.user_id
                FROM (SELECT DISTINCT user_id FROM collection_entry) AS users
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
                               JOIN collection_entry cq
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

    async fn get_price_history(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<PriceHistoryEntry>, AppError> {
        let rows = sqlx::query!(
            r#"
                SELECT date, low, trend, avg, avg1, avg7, avg30
                FROM collection_price_history
                WHERE user_id = $1
                  AND date >= $2
                  AND date <= $3
                ORDER BY date
            "#,
            user_id,
            start_date,
            end_date,
        )
        .fetch_all(&self.pool)
        .await?;

        let entries = rows
            .into_iter()
            .map(|row| PriceHistoryEntry {
                date: row.date,
                price_guide: PriceGuide {
                    low: row.low.into(),
                    trend: row.trend.into(),
                    avg: row.avg.into(),
                    avg1: row.avg1.into(),
                    avg7: row.avg7.into(),
                    avg30: row.avg30.into(),
                },
            })
            .collect();

        Ok(entries)
    }
}

#[cfg(test)]
#[path = "collection_price_history_repository_adapter_tests.rs"]
mod tests;
