use super::*;

fn make_card_entity(rarity: &str, foil: bool, cardmarket_id: Option<i32>) -> CardEntity {
    CardEntity {
        set_code: "FDN".to_string(),
        collector_number: "123".to_string(),
        language_code: "EN".to_string(),
        foil,
        set_name: "Foundations".to_string(),
        name: "Goblin Guide".to_string(),
        rarity: rarity.to_string(),
        quantity: 2,
        purchase_price: 350,
        scryfall_id: Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
        cardmarket_id,
        added_at: None,
    }
}

fn make_card_id_entity(foil: bool) -> CardIdEntity {
    CardIdEntity {
        set_code: "FDN".to_string(),
        collector_number: "123".to_string(),
        language_code: "FR".to_string(),
        foil,
        set_name: "Foundations".to_string(),
        scryfall_id: Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
    }
}

#[test]
fn card_entity_converts_to_card_with_all_fields() {
    let entity = make_card_entity("R", false, Some(42));

    let card: Card = entity.into();

    assert_eq!(card.id.collector_number, "123");
    assert_eq!(card.id.language_code, LanguageCode::EN);
    assert!(!card.id.foil);
    assert_eq!(card.name, "Goblin Guide");
    assert_eq!(card.set_name.name, "Foundations");
    assert_eq!(card.rarity_code, RarityCode::R);
    assert_eq!(card.quantity, 2);
    assert_eq!(card.purchase_price, 350);
    assert_eq!(card.cardmarket_id, Some(42));
}

#[test]
fn card_entity_converts_to_card_without_cardmarket_id() {
    let entity = make_card_entity("C", false, None);

    let card: Card = entity.into();

    assert_eq!(card.cardmarket_id, None);
}

#[test]
fn card_entity_converts_to_card_with_foil_flag() {
    let entity = make_card_entity("U", true, None);

    let card: Card = entity.into();

    assert!(card.id.foil);
}

#[test]
fn card_entity_set_code_is_uppercased_in_card_id_and_set_name() {
    let entity = make_card_entity("M", false, None);

    let card: Card = entity.into();

    assert_eq!(card.id.set_code.to_string(), "FDN");
    assert_eq!(card.set_name.code.to_string(), "FDN");
}

#[test]
fn card_entity_purchase_price_in_cents_is_preserved() {
    let entity = make_card_entity("C", false, None);

    let card: Card = entity.into();

    assert_eq!(card.purchase_price, 350);
}

#[test]
fn from_db_rarity_returns_common_for_c() {
    assert_eq!(from_db_rarity("C"), RarityCode::C);
}

#[test]
fn from_db_rarity_returns_uncommon_for_u() {
    assert_eq!(from_db_rarity("U"), RarityCode::U);
}

#[test]
fn from_db_rarity_returns_rare_for_r() {
    assert_eq!(from_db_rarity("R"), RarityCode::R);
}

#[test]
fn from_db_rarity_returns_mythic_for_m() {
    assert_eq!(from_db_rarity("M"), RarityCode::M);
}

#[test]
fn from_db_rarity_returns_common_for_lowercase() {
    assert_eq!(from_db_rarity("c"), RarityCode::C);
}

#[test]
#[should_panic(expected = "invalid rarity code from database")]
fn from_db_rarity_panics_on_unknown_code() {
    from_db_rarity("X");
}

#[test]
fn card_id_entity_converts_to_card_id_with_foil() {
    let entity = make_card_id_entity(true);

    let card_id: CardId = entity.into();

    assert_eq!(card_id.collector_number, "123");
    assert_eq!(card_id.language_code, LanguageCode::FR);
    assert!(card_id.foil);
    assert_eq!(card_id.set_code.to_string(), "FDN");
}

#[test]
fn card_id_entity_converts_to_card_id_without_foil() {
    let entity = make_card_id_entity(false);

    let card_id: CardId = entity.into();

    assert!(!card_id.foil);
}

#[test]
fn price_as_i32_returns_some_when_value_is_present() {
    let price = Price { value: Some(199) };

    assert_eq!(price.as_i32(), Some(199));
}

#[test]
fn price_as_i32_returns_none_when_value_is_absent() {
    let price = Price::empty();

    assert_eq!(price.as_i32(), None);
}

#[test]
fn price_as_i32_returns_zero_when_value_is_zero() {
    let price = Price { value: Some(0) };

    assert_eq!(price.as_i32(), Some(0));
}

#[test]
fn user_from_id_sets_id_correctly() {
    let user = User::from_id("abc123".to_string());

    assert_eq!(user.id, "abc123");
}

#[test]
fn user_from_id_builds_placeholder_email_from_id() {
    let user = User::from_id("abc123".to_string());

    assert_eq!(user.email, "abc123@placeholder.local");
}

#[test]
fn user_from_id_has_no_name() {
    let user = User::from_id("abc123".to_string());

    assert_eq!(user.name, None);
}

#[test]
fn user_from_id_with_complex_id_builds_correct_email() {
    let user = User::from_id("google|105262637836230123456".to_string());

    assert_eq!(user.id, "google|105262637836230123456");
    assert_eq!(user.email, "google|105262637836230123456@placeholder.local");
}
