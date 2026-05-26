use crate::application::error::AppError;
use crate::application::repository::StatsRepository;
use crate::application::use_case::StatsUseCase;
use crate::domain::stats::Stats;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StatsService {
    repository: Arc<dyn StatsRepository>,
}

impl StatsService {
    pub fn new(repository: Arc<dyn StatsRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl StatsUseCase for StatsService {
    async fn get_stats(&self) -> Result<Stats, AppError> {
        let card_number = self.repository.get_card_number().await?;
        let card_price_number = self.repository.get_card_price_number().await?;
        let db_size_mb = self.repository.get_db_size().await?;

        Ok(Stats {
            card_number,
            card_price_number,
            db_size_mb,
        })
    }
}

#[cfg(test)]
#[path = "stats_service_tests.rs"]
mod tests;
