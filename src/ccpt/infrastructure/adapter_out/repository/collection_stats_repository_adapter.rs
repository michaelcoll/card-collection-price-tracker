use crate::application::error::AppError;
use crate::application::repository::CollectionStatsRepository;
use crate::domain::collection_stats::CollectionStats;
use crate::domain::price::Price;
use crate::domain::set_name::{SetCode, SetName};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct CollectionStatsRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CollectionStatsRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionStatsRepository for CollectionStatsRepositoryAdapter {
    async fn get_collection_stats(&self, user_id: &str) -> Result<CollectionStats, AppError> {
        let totals = sqlx::query!(
            r#"
            SELECT
                COALESCE(SUM(ce.quantity), 0)::BIGINT AS "total_cards!",
                COUNT(*)::BIGINT                       AS "unique_cards!"
            FROM collection_entry ce
            WHERE ce.user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        let prices = sqlx::query!(
            r#"
            SELECT
                MIN(cp.trend)::INT AS price_trend_min,
                MAX(cp.trend)::INT AS price_trend_max
            FROM collection_entry ce
            LEFT JOIN mv_card_prices cp
                ON  cp.set_code         = ce.set_code
                AND cp.collector_number = ce.collector_number
                AND cp.language_code    = ce.language_code
                AND cp.foil             = ce.foil
                AND cp.user_id          = ce.user_id
            WHERE ce.user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        let sets = sqlx::query!(
            r#"
            SELECT DISTINCT sn.set_code, sn.name
            FROM collection_entry ce
            JOIN card c
                ON  c.set_code         = ce.set_code
                AND c.collector_number = ce.collector_number
                AND c.language_code    = ce.language_code
                AND c.foil             = ce.foil
            JOIN set_name sn ON sn.set_code = c.set_code
            WHERE ce.user_id = $1
            ORDER BY sn.name ASC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(CollectionStats {
            total_cards: totals.total_cards as u64,
            unique_cards: totals.unique_cards as u64,
            price_trend_min: prices
                .price_trend_min
                .map(|v| Price::from_cents(v as u32))
                .unwrap_or_else(Price::empty),
            price_trend_max: prices
                .price_trend_max
                .map(|v| Price::from_cents(v as u32))
                .unwrap_or_else(Price::empty),
            sets: sets
                .into_iter()
                .map(|r| SetName::new(SetCode::new(r.set_code), r.name))
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn returns_zeros_for_empty_collection(pool: PgPool) {
        let adapter = CollectionStatsRepositoryAdapter::new(pool);
        let result = adapter.get_collection_stats("unknown-user").await;
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_cards, 0);
        assert_eq!(stats.unique_cards, 0);
        assert!(stats.price_trend_min.value.is_none());
        assert!(stats.price_trend_max.value.is_none());
        assert!(stats.sets.is_empty());
    }

    #[sqlx::test]
    async fn returns_correct_totals(pool: PgPool) {
        sqlx::query("INSERT INTO set_name (set_code, name) VALUES ('TST', 'Test Set')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id) VALUES ('TST', '1', 'en', false, 'Card A', 'C', '12345678-1234-1234-1234-123456789012')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id) VALUES ('TST', '2', 'en', false, 'Card B', 'R', '22345678-1234-1234-1234-123456789012')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO collection_entry (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price) VALUES ('TST', '1', 'en', false, 'user-1', 3, 100)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO collection_entry (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price) VALUES ('TST', '2', 'en', false, 'user-1', 2, 200)")
            .execute(&pool)
            .await
            .unwrap();

        let adapter = CollectionStatsRepositoryAdapter::new(pool);
        let result = adapter.get_collection_stats("user-1").await;
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_cards, 5);
        assert_eq!(stats.unique_cards, 2);
        assert_eq!(stats.sets.len(), 1);
        assert_eq!(stats.sets[0].name, "Test Set");
        assert_eq!(stats.sets[0].code.to_string(), "TST");
    }

    #[sqlx::test]
    async fn does_not_return_other_users_cards(pool: PgPool) {
        sqlx::query("INSERT INTO set_name (set_code, name) VALUES ('TST', 'Test Set')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id) VALUES ('TST', '1', 'en', false, 'Card A', 'C', '12345678-1234-1234-1234-123456789012')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO collection_entry (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price) VALUES ('TST', '1', 'en', false, 'user-other', 10, 100)")
            .execute(&pool)
            .await
            .unwrap();

        let adapter = CollectionStatsRepositoryAdapter::new(pool);
        let result = adapter.get_collection_stats("user-1").await;
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_cards, 0);
        assert_eq!(stats.unique_cards, 0);
    }
}
