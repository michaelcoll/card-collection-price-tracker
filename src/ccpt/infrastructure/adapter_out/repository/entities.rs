use crate::domain::card::Card;
use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::{SetCode, SetName};

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
        }
    }
}
