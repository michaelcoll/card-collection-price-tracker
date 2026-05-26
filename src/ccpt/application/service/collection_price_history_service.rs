use crate::application::error::AppError;
use crate::application::repository::CollectionPriceHistoryRepository;
use crate::application::use_case::GetCollectionPriceHistoryUseCase;
use crate::domain::price::PriceHistoryEntry;
use async_trait::async_trait;
use chrono::NaiveDate;
use std::sync::Arc;

pub struct CollectionPriceHistoryService {
    repository: Arc<dyn CollectionPriceHistoryRepository>,
}

impl CollectionPriceHistoryService {
    pub fn new(repository: Arc<dyn CollectionPriceHistoryRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl GetCollectionPriceHistoryUseCase for CollectionPriceHistoryService {
    async fn get_collection_price_history(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<PriceHistoryEntry>, AppError> {
        if start_date > end_date {
            return Err(AppError::WrongFormat(
                "start_date must be before or equal to end_date".to_string(),
            ));
        }
        self.repository
            .get_price_history(user_id, start_date, end_date)
            .await
    }
}

#[cfg(test)]
#[path = "collection_price_history_service_tests.rs"]
mod tests;
