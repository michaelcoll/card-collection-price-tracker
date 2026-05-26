use crate::application::error::AppError;
use crate::application::repository::CardRepository;
use crate::application::use_case::EnqueueCardMarketIdUpdateUseCase;
use crate::domain::card::CardId;
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub struct CardMarketIdEnqueueService {
    card_repository: Arc<dyn CardRepository>,
    sender: UnboundedSender<(CardId, Uuid)>,
    dedup_set: Arc<Mutex<HashSet<CardId>>>,
}

impl CardMarketIdEnqueueService {
    pub fn new(
        card_repository: Arc<dyn CardRepository>,
        sender: UnboundedSender<(CardId, Uuid)>,
        dedup_set: Arc<Mutex<HashSet<CardId>>>,
    ) -> Self {
        Self {
            card_repository,
            sender,
            dedup_set,
        }
    }
}

#[async_trait]
impl EnqueueCardMarketIdUpdateUseCase for CardMarketIdEnqueueService {
    async fn enqueue_pending_updates(&self) -> Result<usize, AppError> {
        let cards = self.card_repository.get_all_without_cardmarket_id().await?;
        let mut enqueued = 0;
        let mut set = self.dedup_set.lock().unwrap();
        for (card_id, scryfall_id) in cards {
            if set.insert(card_id.clone()) {
                if self.sender.send((card_id, scryfall_id)).is_err() {
                    eprintln!("Worker channel closed, cannot enqueue card");
                } else {
                    enqueued += 1;
                }
            }
        }
        Ok(enqueued)
    }
}

#[cfg(test)]
#[path = "cardmarket_id_enqueue_service_tests.rs"]
mod tests;
