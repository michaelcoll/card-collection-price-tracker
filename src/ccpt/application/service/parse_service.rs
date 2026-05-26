use crate::application::error::AppError;
use crate::domain::card::Card;
use crate::domain::language_code::LanguageCode;
use crate::domain::rarity_code::RarityCode;
use crate::domain::set_name::{SetCode, SetName};
use chrono::{DateTime, Utc};
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
        let line_number = index + 1; // +1 car lignes humaines

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
            added_at,
        );
        cards.push(card);
    }
    Ok(cards)
}

#[cfg(test)]
#[path = "parse_service_tests.rs"]
mod tests;
