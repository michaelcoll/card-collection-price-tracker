use crate::application::error::AppError;
use crate::application::repository::StatsRepository;
use crate::infrastructure::adapter_out::repository::entities::{CountEntity, SizeEntity};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct StatsRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl StatsRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StatsRepository for StatsRepositoryAdapter {
    async fn get_card_number(&self) -> Result<u32, AppError> {
        Ok(
            sqlx::query_as!(CountEntity, "SELECT count(*) AS count FROM card")
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap() as u32,
        )
    }

    async fn get_card_price_number(&self) -> Result<u32, AppError> {
        Ok(sqlx::query_as!(
            CountEntity,
            "SELECT count(*) AS count FROM cardmarket_price"
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap() as u32)
    }

    async fn get_db_size(&self) -> Result<u16, AppError> {
        Ok(sqlx::query_as!(
            SizeEntity,
            "SELECT pg_database_size(current_database()) / 1024 / 1024 as size"
        )
        .fetch_one(&self.pool)
        .await?
        .size
        .unwrap() as u16)
    }
}

#[cfg(test)]
#[path = "stats_repository_adapter_tests.rs"]
mod tests;
