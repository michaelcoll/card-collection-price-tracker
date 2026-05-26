use crate::domain::error::CardParsingError;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RarityCode {
    C,
    U,
    R,
    M,
}

impl RarityCode {
    pub fn try_new<S: AsRef<str>>(s: S) -> Result<Self, CardParsingError> {
        let s_ref = s.as_ref();
        match s_ref.to_lowercase().as_str() {
            "common" => Ok(RarityCode::C),
            "uncommon" => Ok(RarityCode::U),
            "rare" => Ok(RarityCode::R),
            "mythic" => Ok(RarityCode::M),
            _ => Err(CardParsingError::InvalidRarityCode(s_ref.to_string())),
        }
    }

    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Self::try_new(s).expect("invalid rarity code")
    }
}

impl Display for RarityCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RarityCode::C => write!(f, "C"),
            RarityCode::U => write!(f, "U"),
            RarityCode::R => write!(f, "R"),
            RarityCode::M => write!(f, "M"),
        }
    }
}

#[cfg(test)]
#[path = "rarity_code_tests.rs"]
mod tests;
