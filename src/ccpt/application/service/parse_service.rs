use crate::application::error::AppError;
use crate::domain::card::{Card, CardId};
use crate::domain::language_code::LanguageCode;
use crate::domain::rarity_code::RarityCode;
use crate::domain::set_name::{SetCode, SetName};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

fn split_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            fields.push(current_field.trim().to_string());
            current_field.clear();
        } else {
            current_field.push(c);
        }
        i += 1;
    }
    fields.push(current_field.trim().to_string());
    fields
}

pub fn parse_cards(csv: &str) -> Result<Vec<Card>, AppError> {
    let mut cards = Vec::new();

    if csv.lines().count() <= 1 {
        return Err(AppError::WrongFormat(
            "missing headers or empty file".to_string(),
        ));
    }

    for (index, line) in csv.lines().skip(1).enumerate() {
        let line_number = index + 1 + 1; // +1 car lignes humaines, +1 car header

        let fields: Vec<String> = split_line(line);
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();

        if fields.len() == 15 {
            return Err(AppError::WrongFormat(
                "expecting a collection export, got a binder export".to_string(),
            ));
        }

        if fields.len() != 18 {
            return Err(AppError::WrongFormat(format!(
                "expected 18 fields per line, got {}",
                fields.len()
            )));
        }

        let name = field_refs[2];
        let set_code = SetCode::try_new(field_refs[3]).map_err(|_| AppError::ParseError {
            line: line_number,
            field: "set_code",
            value: field_refs[3].to_string(),
        })?;
        let set_name = SetName {
            code: set_code.clone(),
            name: field_refs[4].to_string(),
        };

        let collector_number = field_refs[5];

        let rarity_code = RarityCode::try_new(field_refs[7]).map_err(|_| AppError::ParseError {
            line: line_number,
            field: "rarity",
            value: field_refs[7].to_string(),
        })?;

        let language_code: LanguageCode =
            LanguageCode::try_new(field_refs[15]).map_err(|_| AppError::ParseError {
                line: line_number,
                field: "language_code",
                value: field_refs[15].to_string(),
            })?;
        let foil: bool = field_refs[6] != "normal";

        let quantity: u8 = field_refs[8].parse().map_err(|_e| AppError::ParseError {
            line: line_number,
            field: "quantity",
            value: field_refs[8].to_string(),
        })?;

        let scryfall_id = Uuid::parse_str(field_refs[10]).map_err(|_e| AppError::ParseError {
            line: line_number,
            field: "scryfall_id",
            value: field_refs[10].to_string(),
        })?;

        let purchase_price_float: f32 =
            field_refs[11].parse().map_err(|_e| AppError::ParseError {
                line: line_number,
                field: "purchase_price",
                value: field_refs[11].to_string(),
            })?;

        let purchase_price = (purchase_price_float * 100.0).round() as u32;

        let added_at: Option<DateTime<Utc>> = {
            let raw = field_refs[17];
            if raw.is_empty() {
                None
            } else {
                Some(
                    DateTime::parse_from_rfc3339(raw)
                        .map(|dt| dt.with_timezone(&Utc))
                        .map_err(|_e| AppError::ParseError {
                            line: line_number,
                            field: "added_at",
                            value: raw.to_string(),
                        })?,
                )
            }
        };

        CardId::try_new(
            set_code.clone(),
            collector_number,
            language_code.clone(),
            foil,
        )
        .map_err(|e| AppError::ParseError {
            line: line_number,
            field: "collector_number",
            value: String::from(e),
        })?;

        let card = Card::new_full(
            set_code,
            set_name.name.clone(),
            collector_number,
            language_code,
            foil,
            name,
            rarity_code,
            quantity,
            purchase_price,
            scryfall_id,
            None,
            None,
            added_at,
        );
        cards.push(card);
    }

    let mut seen: HashMap<CardId, Card> = HashMap::new();
    let mut order: Vec<CardId> = Vec::new();
    for card in cards {
        if let Some(existing) = seen.get_mut(&card.id) {
            let new_qty = existing.quantity as u32 + card.quantity as u32;
            let total_cost = existing.purchase_price * existing.quantity as u32
                + card.purchase_price * card.quantity as u32;
            existing.purchase_price = total_cost / new_qty;
            existing.quantity = new_qty.min(u8::MAX as u32) as u8;
            existing.added_at = match (existing.added_at, card.added_at) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
        } else {
            order.push(card.id.clone());
            seen.insert(card.id.clone(), card);
        }
    }

    Ok(order
        .into_iter()
        .map(|id| seen.remove(&id).unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
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
                line: 2,
                field: "set_code",
                value: _
            })
        ));
    }

    #[test]
    fn import_cards_returns_error_for_invalid_language_code() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,\"Brigid, Clachan's Heart // Brigid, Doun's Mind\",ECL,Lorwyn Eclipsed,7,normal,rare,1,110841,cb7d5bbb-4f68-4e38-8bb0-a95af21b24c8,1.75,false,false,near_mint,xx,EUR,2026-02-05T20:44:45.815Z";

        let result = parse_cards(csv);
        assert!(matches!(
            result,
            Err(AppError::ParseError {
                line: 2,
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
                line: 3,
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
                line: 2,
                field: "purchase_price",
                value: _
            })
        ));
    }

    #[test]
    fn import_cards_returns_error_for_too_long_collector_number() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Goblin Boarders,FDN,Foundations,12345678901,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(AppError::ParseError {
                line: 2,
                field: "collector_number",
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
    fn import_cards_deduplicates_by_set_code_collector_number_language_foil() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z\n\
                   My Deck,deck,Goblin Boarders,FDN,Foundations,87,normal,common,2,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.10,false,false,near_mint,fr,EUR,2026-03-01T10:00:00.000Z";

        let cards = parse_cards(csv).unwrap();

        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].id.set_code, SetCode::new("FDN"));
        assert_eq!(cards[0].id.collector_number, "87");
        assert_eq!(cards[0].quantity, 5);
        // weighted average: (3*8 + 2*10) / 5 = (24+20)/5 = 44/5 = 8
        assert_eq!(cards[0].purchase_price, 8);
        // earliest date kept
        assert_eq!(
            cards[0].added_at.unwrap().to_rfc3339(),
            "2026-02-05T20:44:45.815+00:00"
        );
    }

    #[test]
    fn import_cards_returns_error_for_invalid_date_format() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
                   bulk,binder,Repeal,GPT,Guildpact,32,normal,common,2,27563,9e7dd929-4bba-46a6-86c9-b8ed853eb721,0.17,false,false,near_mint,fr,EUR,NOT_A_DATE";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(AppError::ParseError {
                line: 2,
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
}
