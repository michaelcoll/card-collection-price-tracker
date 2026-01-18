use crate::domain::error::CardParsingError;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanguageCode {
    FR,
    EN,
}

impl FromStr for LanguageCode {
    type Err = CardParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FR" => Ok(LanguageCode::FR),
            "EN" => Ok(LanguageCode::EN),
            _ => Err(CardParsingError::InvalidLanguageCode(format!(
                "invalid language code : {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_returns_fr_for_valid_fr_code() {
        let result = LanguageCode::from_str("FR");
        assert_eq!(result, Ok(LanguageCode::FR));
    }

    #[test]
    fn from_str_returns_en_for_valid_en_code() {
        let result = LanguageCode::from_str("EN");
        assert_eq!(result, Ok(LanguageCode::EN));
    }

    #[test]
    fn from_str_is_case_insensitive() {
        let result = LanguageCode::from_str("fr");
        assert_eq!(result, Ok(LanguageCode::FR));

        let result = LanguageCode::from_str("en");
        assert_eq!(result, Ok(LanguageCode::EN));
    }

    #[test]
    fn from_str_returns_error_for_invalid_code() {
        let result = LanguageCode::from_str("DE");
        assert!(matches!(
            result,
            Err(CardParsingError::InvalidLanguageCode(msg)) if msg == "invalid language code : DE"
        ));
    }

    #[test]
    fn from_str_returns_error_for_empty_string() {
        let result = LanguageCode::from_str("");
        assert!(matches!(
            result,
            Err(CardParsingError::InvalidLanguageCode(msg)) if msg == "invalid language code : "
        ));
    }
}
