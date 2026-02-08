use crate::application::error::AppError;
use crate::domain::card::CardId;
use crate::domain::price::Price;
use async_trait::async_trait;

use crate::domain::cardmarket::PriceGuides;
#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardPriceCaller: Send + Sync {
    async fn get_price_by_card_id(&self, id: CardId) -> Result<Price, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardMarketCaller: Send + Sync {
    async fn get_price_guides(&self) -> Result<PriceGuides, AppError>;
}
