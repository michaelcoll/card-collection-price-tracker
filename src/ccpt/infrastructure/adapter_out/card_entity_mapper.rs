use crate::domain::card::Card;
use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::{SetCode, SetName};
use crate::infrastructure::adapter_out::entities::CardEntity;

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
            quantity: entity.quantity as u8,
            purchase_price: entity.purchase_price as u32,
        }
    }
}
