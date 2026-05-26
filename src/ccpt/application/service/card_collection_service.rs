use crate::application::error::AppError;
use crate::application::repository::CollectionPriceHistoryRepository;
use crate::application::use_case::CardCollectionPriceCalculationUseCase;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CardCollectionService {
    card_collection_repository: Arc<dyn CollectionPriceHistoryRepository>,
}

impl CardCollectionService {
    pub fn new(card_collection_repository: Arc<dyn CollectionPriceHistoryRepository>) -> Self {
        Self {
            card_collection_repository,
        }
    }
}

#[async_trait]
impl CardCollectionPriceCalculationUseCase for CardCollectionService {
    async fn calculate_total_price(&self) -> Result<(), AppError> {
        println!("Calculating total price...");

        let dates_and_users = self
            .card_collection_repository
            .get_date_and_user_to_update()
            .await?;

        for (date, user) in dates_and_users {
            self.card_collection_repository
                .update_for_date_and_user(date, user)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "card_collection_service_tests.rs"]
mod tests;
