use crate::application::error::AppError;
use crate::domain::card::{CardId, CardInfo};
use crate::domain::price::{FullPriceGuide, PriceGuide};
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

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

#[async_trait]
#[cfg_attr(test, automock)]
pub trait EdhRecCaller: Send + Sync {
    async fn get_card_info(&self, card_name: String) -> Result<CardInfo, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait ScryfallCaller: Send + Sync {
    async fn get_card_market_id(&self, id: Uuid) -> Result<Option<u32>, AppError>;
}
