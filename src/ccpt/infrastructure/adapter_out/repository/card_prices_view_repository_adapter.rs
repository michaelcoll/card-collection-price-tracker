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
#[path = "card_prices_view_repository_adapter_tests.rs"]
mod tests;
