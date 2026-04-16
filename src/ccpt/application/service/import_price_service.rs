use crate::application::caller::CardMarketCaller;
use crate::application::error::AppError;
use crate::application::repository::{CardMarketPriceRepository, CardPricesViewRepository};
use crate::application::use_case::{CardCollectionPriceCalculationUseCase, ImportPriceUseCase};
use async_trait::async_trait;
use colored::Colorize;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ImportPriceService {
    cardmarket_caller: Arc<dyn CardMarketCaller>,
    cardmarket_repository: Arc<dyn CardMarketPriceRepository>,
    card_prices_view_repository: Arc<dyn CardPricesViewRepository>,
    price_calculation: Arc<dyn CardCollectionPriceCalculationUseCase>,
}

impl ImportPriceService {
    pub fn new(
        cardmarket_caller: Arc<dyn CardMarketCaller>,
        cardmarket_repository: Arc<dyn CardMarketPriceRepository>,
        card_prices_view_repository: Arc<dyn CardPricesViewRepository>,
        price_calculation: Arc<dyn CardCollectionPriceCalculationUseCase>,
    ) -> Self {
        Self {
            cardmarket_caller,
            cardmarket_repository,
            card_prices_view_repository,
            price_calculation,
        }
    }
}

#[async_trait]
impl ImportPriceUseCase for ImportPriceService {
    async fn import_prices_for_current_date(&self) -> Result<(), AppError> {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let (date, price_guides) = self.cardmarket_caller.get_price_guides().await?;
        let price_count = price_guides.len();
        self.cardmarket_repository.save(date, price_guides).await?;
        self.card_prices_view_repository.refresh().await?;
        self.price_calculation.calculate_total_price().await?;

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let duration = end - start;

        println!(
            "{} Imported {} prices for {} in {} ms",
            "ℹ".yellow().bold(),
            price_count,
            &date,
            duration.as_millis()
        );

        Ok(())
    }
}
