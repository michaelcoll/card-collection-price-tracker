use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::{SetCode, SetName};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
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
            "{} {} {} {}",
            self.collector_number,
            self.set_code,
            if self.foil { "★" } else { "·" },
            self.language_code,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub id: CardId,
    pub set_name: SetName,
    pub name: String,
    pub quantity: u8,
    /// Price in cents
    pub purchase_price: u32,
}

impl Card {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        set_code: impl Into<SetCode>,
        set_name: impl Into<String>,
        collector_number: impl Into<String>,
        language_code: LanguageCode,
        foil: bool,
        name: impl Into<String>,
        quantity: u8,
        purchase_price: u32,
    ) -> Self {
        let set_code: SetCode = set_code.into();
        let set_name = SetName::new(set_code.clone(), set_name);
        Card {
            id: CardId::new(set_code, collector_number.into(), language_code, foil),
            set_name,
            name: name.into(),
            quantity,
            purchase_price,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardInfo {
    pub inclusion: u32,
    pub total_decks: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::language_code::LanguageCode;

    #[test]
    fn display_card_id_with_foil() {
        let card_id = CardId::new("FDN", "123", LanguageCode::EN, true);
        assert_eq!(card_id.to_string(), "123 FDN ★ EN");
    }

    #[test]
    fn display_card_id_without_foil() {
        let card_id = CardId::new("FDN", "456", LanguageCode::FR, false);
        assert_eq!(card_id.to_string(), "456 FDN · FR");
    }

    #[test]
    fn card_equality_same_values() {
        let card1 = Card::new(
            "ECL",
            "Lorwyn Eclipsed",
            "1",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            2,
            1000,
        );

        let card2 = card1.clone();
        assert_eq!(card1, card2);
    }

    #[test]
    fn card_equality_different_values() {
        let card1 = Card::new(
            "ECL",
            "Lorwyn Eclipsed",
            "1",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            2,
            1000,
        );

        let card2 = Card::new(
            "FND",
            "Foundations",
            "2",
            LanguageCode::FR,
            true,
            "Goblin Boarders",
            1,
            2000,
        );

        assert_ne!(card1, card2);
    }
}
