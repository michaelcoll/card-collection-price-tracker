use crate::application::service::error::ImportError;
use crate::domain::card::{Card, CardId};
use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::{SetCode, SetName};

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

pub(crate) fn parse_cards(csv: &str) -> Result<Vec<Card>, ImportError> {
    let mut cards = Vec::new();

    if csv.lines().count() <= 1 {
        return Err(ImportError::WrongFormat(
            "missing headers or empty file".to_string(),
        ));
    }

    for line in csv.lines().skip(1) {
        let fields: Vec<String> = split_line(line);
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();

        if fields.len() == 15 {
            return Err(ImportError::WrongFormat(
                "expecting a collection export, got a binder export".to_string(),
            ));
        }

        if fields.len() != 17 {
            return Err(ImportError::WrongFormat(format!(
                "expected 17 fields per line, got {}",
                fields.len()
            )));
        }

        let set_code = SetCode::new(field_refs[3])?;
        let set_name = SetName {
            code: set_code.clone(),
            name: field_refs[4].to_string(),
        };

        let collector_number: u16 = field_refs[5].parse()?;
        let language_code: LanguageCode = field_refs[15].parse()?;
        let foil: bool = field_refs[6] != "normal";
        let quantity: u8 = field_refs[8].parse()?;
        let purchase_price_float: f32 = field_refs[11].parse()?;
        let purchase_price = (purchase_price_float * 100.0).round() as u32;

        let id = CardId {
            set_code,
            collector_number,
            language_code,
            foil,
        };
        let card = Card {
            id,
            set_name,
            quantity,
            purchase_price,
        };
        cards.push(card);
    }
    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::SetCode;

    #[test]
    fn import_cards_parses_valid_csv() -> Result<(), ImportError> {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
                   bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR\n\
                   bulk,binder,Repeal,GPT,Guildpact,32,normal,common,2,27563,9e7dd929-4bba-46a6-86c9-b8ed853eb721,0.17,false,false,near_mint,fr,EUR\n\
                   bulk,binder,\"Dwynen, Gilt-Leaf Daen\",FDN,Foundations,217,normal,uncommon,2,100086,01c00d7b-7fac-4f8c-a1ea-de2cf4d06627,0.2,false,false,near_mint,fr,EUR";

        let cards = parse_cards(csv)?;

        assert_eq!(cards.len(), 3);

        assert_eq!(cards[0].id.set_code, SetCode::new("FDN")?);
        assert_eq!(cards[0].id.collector_number, 87);
        assert_eq!(cards[0].id.language_code, LanguageCode::FR);
        assert!(!cards[0].id.foil);
        assert_eq!(cards[0].quantity, 3);
        assert_eq!(cards[0].purchase_price, 8);

        assert_eq!(cards[1].id.set_code, SetCode::new("GPT")?);
        assert_eq!(cards[1].id.collector_number, 32);
        assert_eq!(cards[1].id.language_code, LanguageCode::FR);
        assert!(!cards[1].id.foil);
        assert_eq!(cards[1].quantity, 2);
        assert_eq!(cards[1].purchase_price, 17);

        assert_eq!(cards[2].id.set_code, SetCode::new("FDN")?);
        assert_eq!(cards[2].id.collector_number, 217);
        assert_eq!(cards[2].id.language_code, LanguageCode::FR);
        assert!(!cards[2].id.foil);
        assert_eq!(cards[2].quantity, 2);
        assert_eq!(cards[2].purchase_price, 20);

        Ok(())
    }

    #[test]
    fn import_cards_returns_error_for_invalid_set_code() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
                   bulk,binder,\"Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight\",ECLD,Lorwyn Eclipsed,13,normal,mythic,1,108961,b2d9d5ca-7e15-437a-bdfc-5972b42148fe,12.35,false,false,near_mint,fr,EUR";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(ImportError::WrongFormat(err)) if err == "set code must be exactly 3 characters (got ECLD)"
        ));
    }

    #[test]
    fn import_cards_returns_error_for_invalid_language_code() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
                   bulk,binder,\"Brigid, Clachan's Heart // Brigid, Doun's Mind\",ECL,Lorwyn Eclipsed,7,normal,rare,1,110841,cb7d5bbb-4f68-4e38-8bb0-a95af21b24c8,1.75,false,false,near_mint,de,EUR";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(ImportError::WrongFormat(err)) if err == "invalid language code : de"
        ));
    }

    #[test]
    fn import_cards_returns_error_for_invalid_number_format() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
                   bulk,binder,Stormshriek Feral // Flush Out,TDM,Tarkir: Dragonstorm,FAKE_COLLECTION_NUMBER,normal,common,1,104447,0ec92c44-7cf0-48a5-a3ca-bc633496d887,0.11,false,false,near_mint,fr,EUR";

        let result = parse_cards(csv);

        assert!(matches!(result, Err(ImportError::ParseError())));
    }

    #[test]
    fn import_cards_handles_empty_csv() {
        let csv = "";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(ImportError::WrongFormat(err)) if err == "missing headers or empty file"
        ));
    }

    #[test]
    fn import_cards_returns_error_for_binder_export_with_15_columns() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition\n\
               bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(ImportError::WrongFormat(err)) if err == "expecting a collection export, got a binder export"
        ));
    }

    #[test]
    fn import_cards_returns_error_for_wrong_column_count() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID\n\
               bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(ImportError::WrongFormat(err)) if err.contains("expected 17 fields per line")
        ));
    }

    #[test]
    fn import_cards_returns_error_for_too_many_columns() {
        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Extra Column\n\
               bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR,extra";

        let result = parse_cards(csv);

        assert!(matches!(
            result,
            Err(ImportError::WrongFormat(err)) if err.contains("expected 17 fields per line")
        ));
    }
}
