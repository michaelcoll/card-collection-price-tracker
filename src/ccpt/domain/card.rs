use crate::domain::language_code::LanguageCode;
use crate::domain::price::PriceGuide;
use crate::domain::rarity_code::RarityCode;
use crate::domain::set_name::{SetCode, SetName};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CardId {
    pub set_code: SetCode,
    pub collector_number: String,
    pub language_code: LanguageCode,
    pub foil: bool,
}

impl CardId {
    pub fn new(
        set_code: impl Into<SetCode>,
        collector_number: impl Into<String>,
        language_code: LanguageCode,
        foil: bool,
    ) -> Self {
        CardId {
            set_code: set_code.into(),
            collector_number: collector_number.into(),
            language_code,
            foil,
        }
    }
}

impl Display for CardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>5} {} {} {}",
            self.collector_number,
            self.set_code,
            if self.foil { "⭑" } else { "·" },
            self.language_code,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub id: CardId,
    pub set_name: SetName,
    pub name: String,
    pub rarity_code: RarityCode,
    pub quantity: u8,
    /// Price in cents
    pub purchase_price: u32,
    pub added_at: Option<chrono::DateTime<chrono::Utc>>,
    pub scryfall_id: uuid::Uuid,
    pub cardmarket_id: Option<u32>,
    pub price_guide: Option<PriceGuide>,
}

impl Card {
    #[allow(clippy::too_many_arguments)]
    #[allow(unused)]
    pub fn new(
        set_code: impl Into<SetCode>,
        set_name: impl Into<String>,
        collector_number: impl Into<String>,
        language_code: LanguageCode,
        foil: bool,
        name: impl Into<String>,
        rarity_code: RarityCode,
        quantity: u8,
        purchase_price: u32,
    ) -> Self {
        let set_code: SetCode = set_code.into();
        let set_name = SetName::new(set_code.clone(), set_name);
        Card {
            id: CardId::new(set_code, collector_number.into(), language_code, foil),
            set_name,
            name: name.into(),
            rarity_code,
            quantity,
            purchase_price,
            scryfall_id: uuid::Uuid::default(),
            cardmarket_id: None,
            added_at: None,
            price_guide: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_full(
        set_code: impl Into<SetCode>,
        set_name: impl Into<String>,
        collector_number: impl Into<String>,
        language_code: LanguageCode,
        foil: bool,
        name: impl Into<String>,
        rarity_code: RarityCode,
        quantity: u8,
        purchase_price: u32,
        scryfall_id: uuid::Uuid,
        cardmarket_id: Option<u32>,
        added_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        let set_code: SetCode = set_code.into();
        let set_name = SetName::new(set_code.clone(), set_name);
        Card {
            id: CardId::new(set_code, collector_number.into(), language_code, foil),
            set_name,
            name: name.into(),
            rarity_code,
            quantity,
            purchase_price,
            scryfall_id,
            cardmarket_id,
            added_at,
            price_guide: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardInfo {
    pub inclusion: u32,
    pub total_decks: u32,
}

#[cfg(test)]
#[path = "card_tests.rs"]
mod tests;
