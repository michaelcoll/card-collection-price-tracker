use crate::domain::card::Card;
use crate::domain::language_code::LanguageCode;
use crate::domain::price::Price;
use crate::domain::set_name::{SetCode, SetName};
use crate::domain::user::User;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardEntity {
    pub set_code: String,
    pub collector_number: String,
    pub language_code: String,
    pub foil: bool,
    pub set_name: String,
    pub name: String,
    pub quantity: i32,
    /// Price in cents
    pub purchase_price: i32,
    pub scryfall_id: Uuid,
    pub cardmarket_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetNameEntity {
    pub set_code: String,
    pub name: String,
}

impl From<CardEntity> for Card {
    fn from(entity: CardEntity) -> Card {
        let set_code = SetCode::new(entity.set_code);
        Card {
            id: crate::domain::card::CardId {
                set_code: set_code.clone(),
                collector_number: entity.collector_number,
                language_code: LanguageCode::new(entity.language_code),
                foil: entity.foil,
            },
            set_name: SetName {
                code: set_code.clone(),
                name: entity.set_name,
            },
            name: entity.name,
            quantity: entity.quantity as u8,
            purchase_price: entity.purchase_price as u32,
            scryfall_id: entity.scryfall_id,
            cardmarket_id: entity.cardmarket_id.map(|id| id as u32),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardMarketPriceEntity {
    pub id_produit: u32,
    pub date: NaiveDate,
    pub low: Option<u32>,
    pub avg: Option<u32>,
    pub trend: Option<u32>,
    pub avg1: Option<u32>,
    pub avg7: Option<u32>,
    pub avg30: Option<u32>,
    pub low_foil: Option<u32>,
    pub avg_foil: Option<u32>,
    pub trend_foil: Option<u32>,
    pub avg1_foil: Option<u32>,
    pub avg7_foil: Option<u32>,
    pub avg30_foil: Option<u32>,
}

impl Price {
    pub fn as_i32(&self) -> Option<i32> {
        self.value.map(|v| v as i32)
    }
}

impl User {
    pub fn from_id(id: String) -> Self {
        User { id }
    }
}
