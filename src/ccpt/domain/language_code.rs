use crate::domain::error::CardParsingError;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LanguageCode {
    FR,
    EN,
    JA,
    IT,
}

impl LanguageCode {
    pub fn try_new<S: AsRef<str>>(s: S) -> Result<Self, CardParsingError> {
        let s_ref = s.as_ref();
        match s_ref.to_uppercase().as_str() {
            "FR" => Ok(LanguageCode::FR),
            "EN" => Ok(LanguageCode::EN),
            "JA" => Ok(LanguageCode::JA),
            "IT" => Ok(LanguageCode::IT),
            _ => Err(CardParsingError::InvalidLanguageCode(s_ref.to_string())),
        }
    }

    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Self::try_new(s).expect("invalid language code")
    }
}

impl Display for LanguageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageCode::FR => write!(f, "FR"),
            LanguageCode::EN => write!(f, "EN"),
            LanguageCode::JA => write!(f, "JA"),
            LanguageCode::IT => write!(f, "IT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_language_code_with_valid_fr_creates_instance() {
        let code = LanguageCode::new("FR");
        assert_eq!(code, LanguageCode::FR);
    }

    #[test]
    fn new_language_code_with_valid_en_creates_instance() {
        let code = LanguageCode::new("EN");
        assert_eq!(code, LanguageCode::EN);
    }

    #[test]
    fn try_new_language_code_with_invalid_code_returns_error() {
        let result = LanguageCode::try_new("DE");
        assert!(result.is_err());
    }

    #[test]
    fn try_new_language_code_with_invalid_code_contains_msg() {
        let result = LanguageCode::try_new("DE");
        match result {
            Err(CardParsingError::InvalidLanguageCode(msg)) => assert_eq!(msg, "DE"),
            _ => panic!("Expected InvalidLanguageCode variant"),
        }
    }

    #[test]
    fn new_language_code_is_case_insensitive() {
        let code_lower = LanguageCode::new("fr");
        let code_mixed = LanguageCode::new("Fr");
        assert_eq!(code_lower, LanguageCode::FR);
        assert_eq!(code_mixed, LanguageCode::FR);
    }

    #[test]
    fn display_language_code_returns_correct_string() {
        assert_eq!(LanguageCode::FR.to_string(), "FR");
        assert_eq!(LanguageCode::EN.to_string(), "EN");
    }
}
