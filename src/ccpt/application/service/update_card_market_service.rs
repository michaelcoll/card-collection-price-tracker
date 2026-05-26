use crate::application::caller::ScryfallCaller;
use crate::application::repository::{CardPricesViewRepository, CardRepository};
use crate::application::use_case::CardCollectionPriceCalculationUseCase;
use crate::domain::card::CardId;
use colored::Colorize;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

pub struct CardMarketIdWorker {
    card_repository: Arc<dyn CardRepository>,
    scryfall_caller: Arc<dyn ScryfallCaller>,
    price_calculation: Arc<dyn CardCollectionPriceCalculationUseCase>,
    card_prices_view_repository: Arc<dyn CardPricesViewRepository>,
    dedup_set: Arc<Mutex<HashSet<CardId>>>,
}

impl CardMarketIdWorker {
    pub fn new(
        card_repository: Arc<dyn CardRepository>,
        scryfall_caller: Arc<dyn ScryfallCaller>,
        price_calculation: Arc<dyn CardCollectionPriceCalculationUseCase>,
        card_prices_view_repository: Arc<dyn CardPricesViewRepository>,
        dedup_set: Arc<Mutex<HashSet<CardId>>>,
    ) -> Self {
        Self {
            card_repository,
            scryfall_caller,
            price_calculation,
            card_prices_view_repository,
            dedup_set,
        }
    }

    pub async fn run(self, mut receiver: UnboundedReceiver<(CardId, Uuid)>) {
        println!("{} Card market id updater started.", "✔".green().bold());

        while let Some((card_id, scryfall_id)) = receiver.recv().await {
            let cardmarket_id = self.scryfall_caller.get_card_market_id(scryfall_id).await;
            if let Ok(id) = cardmarket_id {
                if let Err(e) = self
                    .card_repository
                    .update_cardmarket_id(card_id.clone(), id)
                    .await
                {
                    eprintln!("Failed to update card with CardMarket ID: {:?}", e);
                } else {
                    println!("Updated card {} with CardMarket ID: {:?}", card_id, id);
                }
            } else if let Err(e) = cardmarket_id {
                eprintln!(
                    "Failed to fetch CardMarket ID for card {}: {:?}",
                    card_id, e
                );
            }

            {
                let mut set = self.dedup_set.lock().unwrap();
                set.remove(&card_id);
            }

            if receiver.is_empty()
                && let Err(e) = self.price_calculation.calculate_total_price().await
            {
                eprintln!("Failed to calculate total price: {:?}", e);
            }

            if receiver.is_empty()
                && let Err(e) = self.card_prices_view_repository.refresh().await
            {
                eprintln!("Failed to refresh card price view: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[path = "update_card_market_service_tests.rs"]
mod tests;
