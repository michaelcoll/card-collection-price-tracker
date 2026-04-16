use crate::application::error::AppError;
use crate::application::repository::CardPricesViewRepository;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct CardPricesViewRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardPricesViewRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardPricesViewRepository for CardPricesViewRepositoryAdapter {
    async fn refresh(&self) -> Result<(), AppError> {
        sqlx::query("REFRESH MATERIALIZED VIEW CONCURRENTLY mv_card_prices")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Pool, Postgres};

    #[sqlx::test]
    async fn test_refresh_materialized_view(pool: Pool<Postgres>) {
        let adapter = CardPricesViewRepositoryAdapter::new(pool);

        let result = adapter.refresh().await;

        assert!(result.is_ok());
    }
}
