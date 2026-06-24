use crate::domain::price::Price;
use crate::domain::set_name::SetName;

pub struct CollectionStats {
    pub total_cards: u64,
    pub unique_cards: u64,
    pub price_trend_min: Price,
    pub price_trend_max: Price,
    pub sets: Vec<SetName>,
}
