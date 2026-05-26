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
#[path = "language_code_tests.rs"]
mod tests;
