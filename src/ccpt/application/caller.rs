use crate::application::error::AppError;
use crate::domain::card::CardId;
use crate::domain::price::Price;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait CardPriceCaller {
    #[allow(dead_code)]
    fn get_price_by_card_id(&mut self, id: CardId) -> Result<Price, AppError>;
}
