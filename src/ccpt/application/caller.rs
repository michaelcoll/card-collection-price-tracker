#[cfg(test)]
use mockall::automock;

use crate::domain::card::CardId;
use crate::domain::price::Price;

#[derive(Debug, PartialEq, Eq)]
pub enum CallerError {
    PriceNotFound,
    CallError(String),
}

impl From<CallerError> for String {
    fn from(val: CallerError) -> String {
        match val {
            CallerError::CallError(msg) => msg,
            CallerError::PriceNotFound => "Price not found".to_string(),
        }
    }
}

#[cfg_attr(test, automock)]
pub trait CardPriceCaller {
    fn get_price_by_card_id(&mut self, id: CardId) -> Result<Price, CallerError>;
}
