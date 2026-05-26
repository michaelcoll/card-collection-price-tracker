use super::*;
use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::SetCode;

#[test]
fn import_cards_parses_valid_csv() -> Result<(), AppError> {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z\n\
                   bulk,binder,Repeal,GPT,Guildpact,32,normal,common,2,27563,9e7dd929-4bba-46a6-86c9-b8ed853eb721,0.17,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z\n\
                   bulk,binder,\"Dwynen, Gilt-Leaf Daen\",FDN,Foundations,217,normal,uncommon,2,100086,01c00d7b-7fac-4f8c-a1ea-de2cf4d06627,0.2,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";

    let cards = parse_cards(csv)?;

    assert_eq!(cards.len(), 3);

    assert_eq!(cards[0].id.set_code, SetCode::new("FDN"));
    assert_eq!(cards[0].id.collector_number, "87");
    assert_eq!(cards[0].id.language_code, LanguageCode::FR);
    assert!(!cards[0].id.foil);
    assert_eq!(cards[0].quantity, 3);
    assert_eq!(cards[0].purchase_price, 8);
    assert!(cards[0].added_at.is_some());

    assert_eq!(cards[1].id.set_code, SetCode::new("GPT"));
    assert_eq!(cards[1].id.collector_number, "32");
    assert_eq!(cards[1].id.language_code, LanguageCode::FR);
    assert!(!cards[1].id.foil);
    assert_eq!(cards[1].quantity, 2);
    assert_eq!(cards[1].purchase_price, 17);

    assert_eq!(cards[2].id.set_code, SetCode::new("FDN"));
    assert_eq!(cards[2].id.collector_number, "217");
    assert_eq!(cards[2].id.language_code, LanguageCode::FR);
    assert!(!cards[2].id.foil);
    assert_eq!(cards[2].quantity, 2);
    assert_eq!(cards[2].purchase_price, 20);

    Ok(())
}

#[test]
fn import_cards_returns_error_for_invalid_set_code() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,\"Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight\",EC,Lorwyn Eclipsed,13,normal,mythic,1,108961,b2d9d5ca-7e15-437a-bdfc-5972b42148fe,12.35,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";

    let result = parse_cards(csv);
    assert!(matches!(
        result,
        Err(AppError::ParseError {
            line: 1,
            field: "set_code",
            value: _
        })
    ));
}

#[test]
fn import_cards_returns_error_for_invalid_language_code() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,\"Brigid, Clachan's Heart // Brigid, Doun's Mind\",ECL,Lorwyn Eclipsed,7,normal,rare,1,110841,cb7d5bbb-4f68-4e38-8bb0-a95af21b24c8,1.75,false,false,near_mint,de,EUR,2026-02-05T20:44:45.815Z";

    let result = parse_cards(csv);
    assert!(matches!(
        result,
        Err(AppError::ParseError {
            line: 1,
            field: "language_code",
            value: _
        })
    ));
}

#[test]
fn import_cards_returns_error_for_invalid_quantity_number_format() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Stormshriek Feral // Flush Out,TDM,Tarkir: Dragonstorm,15,normal,common,1,104447,0ec92c44-7cf0-48a5-a3ca-bc633496d887,0.11,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z\n\
                   bulk,binder,Stormshriek Feral // Flush Out,TDM,Tarkir: Dragonstorm,15,normal,common,NOT_VALID_NUMBER,104447,0ec92c44-7cf0-48a5-a3ca-bc633496d887,0.11,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";

    let result = parse_cards(csv);

    println!("{:?}", result);

    assert!(matches!(
        result,
        Err(AppError::ParseError {
            line: 2,
            field: "quantity",
            value: _,
        })
    ));
}

#[test]
fn import_cards_returns_error_for_invalid_float_format() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Stormshriek Feral // Flush Out,TDM,Tarkir: Dragonstorm,15,normal,common,1,104447,0ec92c44-7cf0-48a5-a3ca-bc633496d887,0a11,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";

    let result = parse_cards(csv);

    assert!(matches!(
        result,
        Err(AppError::ParseError {
            line: 1,
            field: "purchase_price",
            value: _
        })
    ));
}

#[test]
fn import_cards_handles_empty_csv() {
    let csv = "";

    let result = parse_cards(csv);

    assert!(matches!(
        result,
        Err(AppError::WrongFormat(err)) if err == "missing headers or empty file"
    ));
}

#[test]
fn import_cards_returns_error_for_binder_export_with_15_columns() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition\n\
               bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint";

    let result = parse_cards(csv);

    assert!(matches!(
        result,
        Err(AppError::WrongFormat(err)) if err == "expecting a collection export, got a binder export"
    ));
}

#[test]
fn import_cards_returns_error_for_wrong_column_count() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID\n\
               bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506";

    let result = parse_cards(csv);

    assert!(matches!(
        result,
        Err(AppError::WrongFormat(err)) if err.contains("expected 18 fields per line")
    ));
}

#[test]
fn import_cards_returns_error_for_invalid_date_format() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Repeal,GPT,Guildpact,32,normal,common,2,27563,9e7dd929-4bba-46a6-86c9-b8ed853eb721,0.17,false,false,near_mint,fr,EUR,NOT_A_DATE";

    let result = parse_cards(csv);

    assert!(matches!(
        result,
        Err(AppError::ParseError {
            line: 1,
            field: "added_at",
            value: _,
        })
    ));
}

#[test]
fn import_cards_is_valid_with_alphanum_collection_number() {
    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
               bulk,binder,\"Felothar, Dawn of the Abzan\",PTDM,Tarkir: Dragonstorm Promos,184s,foil,rare,1,105214,09478378-c28b-4334-a0a1-157325ed8e5b,0.76,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";

    let result = parse_cards(csv);

    let card = Card::new_full(
        "PTDM",
        "Tarkir: Dragonstorm Promos",
        "184s",
        LanguageCode::FR,
        true,
        "Felothar, Dawn of the Abzan",
        RarityCode::R,
        1,
        76,
        Uuid::parse_str("09478378-c28b-4334-a0a1-157325ed8e5b").unwrap(),
        None,
        Some(
            DateTime::parse_from_rfc3339("2026-02-05T20:44:45.815Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
    );

    assert_eq!(result.clone().unwrap().len(), 1);
    assert_eq!(result.clone().unwrap()[0], card);
}
