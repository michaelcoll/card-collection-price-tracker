use crate::domain::card::{Card, CardId};
use crate::domain::language_code::LanguageCode;
use crate::domain::price::{Price, PriceGuide};
use crate::domain::rarity_code::RarityCode;
use crate::domain::set_name::{SetCode, SetName};
use crate::domain::user::User;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardEntity {
    pub set_code: String,
    pub collector_number: String,
    pub language_code: String,
    pub foil: bool,
    pub set_name: String,
    pub name: String,
    pub rarity: String,
    pub quantity: i32,
    /// Price in cents
    pub purchase_price: i32,
    pub added_at: Option<DateTime<Utc>>,
    pub scryfall_id: Uuid,
    pub cardmarket_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardIdEntity {
    pub set_code: String,
    pub collector_number: String,
    pub language_code: String,
    pub foil: bool,
    pub set_name: String,
    pub scryfall_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetNameEntity {
    pub set_code: String,
    pub name: String,
}

impl From<CardEntity> for Card {
    fn from(entity: CardEntity) -> Card {
        let set_code =
            SetCode::try_new(entity.set_code).expect("database contains invalid set_code");
        Card {
            id: CardId {
                set_code: set_code.clone(),
                collector_number: entity.collector_number,
                language_code: LanguageCode::try_new(entity.language_code)
                    .expect("database contains invalid language_code"),
                foil: entity.foil,
            },
            set_name: SetName {
                code: set_code.clone(),
                name: entity.set_name,
            },
            name: entity.name,
            rarity_code: from_db_rarity(entity.rarity),
            quantity: entity.quantity as u8,
            purchase_price: entity.purchase_price as u32,
            added_at: entity.added_at,
            scryfall_id: entity.scryfall_id,
            cardmarket_id: entity.cardmarket_id.map(|id| id as u32),
            price_guide: None,
        }
    }
}

fn from_db_rarity<S: AsRef<str>>(s: S) -> RarityCode {
    let s = s.as_ref().to_uppercase();
    match s.as_str() {
        "C" | "c" => RarityCode::C,
        "U" | "u" => RarityCode::U,
        "R" | "r" => RarityCode::R,
        "M" | "m" => RarityCode::M,
        _ => panic!("invalid rarity code from database: {}", s),
    }
}

impl From<CardIdEntity> for CardId {
    fn from(entity: CardIdEntity) -> CardId {
        let set_code =
            SetCode::try_new(entity.set_code).expect("database contains invalid set_code");
        CardId {
            set_code: set_code.clone(),
            collector_number: entity.collector_number,
            language_code: LanguageCode::try_new(entity.language_code)
                .expect("database contains invalid language_code"),
            foil: entity.foil,
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
        User {
            id: id.clone(),
            email: format!("{}@placeholder.local", id),
            name: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountEntity {
    pub count: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeEntity {
    pub size: Option<i64>,
}

#[derive(sqlx::FromRow)]
pub struct CardWithPriceEntity {
    pub set_code: String,
    pub set_name: String,
    pub collector_number: String,
    pub language_code: String,
    pub foil: bool,
    pub name: String,
    pub rarity: String,
    pub scryfall_id: Uuid,
    pub quantity: i32,
    pub purchase_price: i32,
    pub avg: Option<i32>,
    pub low: Option<i32>,
    pub trend: Option<i32>,
    pub avg1: Option<i32>,
    pub avg7: Option<i32>,
    pub avg30: Option<i32>,
}

impl From<i32> for Price {
    fn from(value: i32) -> Self {
        Price::from_cents(value as u32)
    }
}

impl From<Option<i32>> for Price {
    fn from(value: Option<i32>) -> Self {
        value
            .map(|v| v as u32)
            .map(Price::from_cents)
            .unwrap_or_else(Price::empty)
    }
}

impl From<CardWithPriceEntity> for Card {
    fn from(e: CardWithPriceEntity) -> Self {
        let price_guide = if e.avg.is_some() || e.low.is_some() {
            Some(PriceGuide::new(
                e.low, e.trend, e.avg, e.avg1, e.avg7, e.avg30,
            ))
        } else {
            None
        };

        let set_code = SetCode::try_new(&e.set_code).expect("database contains invalid set_code");
        Card {
            id: CardId::new(
                set_code.clone(),
                e.collector_number,
                LanguageCode::try_new(&e.language_code)
                    .expect("database contains invalid language_code"),
                e.foil,
            ),
            set_name: SetName::new(set_code, e.set_name),
            name: e.name,
            rarity_code: from_db_rarity(e.rarity),
            scryfall_id: e.scryfall_id,
            cardmarket_id: None,
            added_at: None,
            quantity: e.quantity as u8,
            purchase_price: e.purchase_price as u32,
            price_guide,
        }
    }
}

#[cfg(test)]
#[path = "entities_tests.rs"]
mod tests;
