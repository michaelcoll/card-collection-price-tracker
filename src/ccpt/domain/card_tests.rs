use super::*;
use crate::domain::language_code::LanguageCode;

#[test]
fn display_card_id_with_foil() {
    let card_id = CardId::new("FDN", "123", LanguageCode::EN, true);
    assert_eq!(card_id.to_string(), "  123 FDN ⭑ EN");
}

#[test]
fn display_card_id_with_foil_and_collection_number_on_one_digit() {
    let card_id = CardId::new("FDN", "3", LanguageCode::EN, true);
    assert_eq!(card_id.to_string(), "    3 FDN ⭑ EN");
}

#[test]
fn display_card_id_without_foil() {
    let card_id = CardId::new("FDN", "456", LanguageCode::FR, false);
    assert_eq!(card_id.to_string(), "  456 FDN · FR");
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
        RarityCode::C,
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
        RarityCode::C,
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
        RarityCode::C,
        1,
        2000,
    );

    assert_ne!(card1, card2);
}
