use crate::application::error::AppError;
use crate::domain::card::CardId;
use crate::domain::price::{FullPriceGuide, PriceGuide};
use async_trait::async_trait;
use chrono::NaiveDate;
#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardPriceCaller: Send + Sync {
    async fn get_price_by_card_id(&self, id: CardId) -> Result<PriceGuide, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardMarketCaller: Send + Sync {
    async fn get_price_guides(&self) -> Result<(NaiveDate, Vec<FullPriceGuide>), AppError>;
}
