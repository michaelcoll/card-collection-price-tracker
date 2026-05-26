use crate::application::error::AppError;
use crate::application::repository::{CardPricesViewRepository, CardRepository, SetNameRepository};
use crate::application::service::parse_service::parse_cards;
use crate::application::use_case::{EnqueueCardMarketIdUpdateUseCase, ImportCardUseCase};
use crate::domain::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImportCardService {
    card_repository: Arc<dyn CardRepository>,
    set_name_repository: Arc<dyn SetNameRepository>,
    enqueue_cardmarket_ids: Arc<dyn EnqueueCardMarketIdUpdateUseCase>,
    card_prices_view_repository: Arc<dyn CardPricesViewRepository>,
}

impl ImportCardService {
    pub fn new(
        card_repository: Arc<dyn CardRepository>,
        set_name_repository: Arc<dyn SetNameRepository>,
        enqueue_cardmarket_ids: Arc<dyn EnqueueCardMarketIdUpdateUseCase>,
        card_prices_view_repository: Arc<dyn CardPricesViewRepository>,
    ) -> Self {
        Self {
            card_repository,
            set_name_repository,
            enqueue_cardmarket_ids,
            card_prices_view_repository,
        }
    }
}

#[async_trait]
impl ImportCardUseCase for ImportCardService {
    async fn import_cards(&self, csv: &str, user: User) -> Result<(), AppError> {
        let cards = parse_cards(csv)?;

        self.card_repository.delete_all(user.clone()).await?;

        for card in cards {
            if !self
                .set_name_repository
                .exists_by_code(card.id.set_code.clone())
                .await?
            {
                let set_name = card.set_name.clone();
                self.set_name_repository.save(set_name).await?;
            }

            self.card_repository.save(user.clone(), card).await?;
        }

        self.enqueue_cardmarket_ids
            .enqueue_pending_updates()
            .await?;
        self.card_prices_view_repository.refresh().await?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "import_card_service_tests.rs"]
mod tests;
