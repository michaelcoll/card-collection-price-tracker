use crate::domain::card::Card;
use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::{SetCode, SetName};
use crate::infrastructure::adapter_out::entities::CardEntity;
use sqlx::Row;

impl From<CardEntity> for Card {
    fn from(entity: CardEntity) -> Card {
        let set_code = SetCode::new(entity.set_code);
        Card {
            id: crate::domain::card::CardId {
                set_code: set_code.clone(),
                collector_number: entity.collector_number as u16,
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

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for SetName {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<SetName, sqlx::Error> {
        let set_code_str: String = row.try_get::<String, _>("set_code")?;
        let set_code = SetCode::new(&set_code_str);
        let name: String = row.try_get::<String, _>("name")?;

        Ok(SetName {
            code: set_code,
            name,
        })
    }
}
