use crate::domain::error::CardParsingError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetCode(String);

impl SetCode {
    pub fn try_new(s: impl Into<String>) -> Result<Self, CardParsingError> {
        let name = s.into().to_uppercase();
        if name.chars().count() >= 3 && name.chars().count() <= 5 {
            Ok(SetCode(name))
        } else {
            Err(CardParsingError::InvalidSetCode(format!(
                "set code must be between 3 and 5 characters (got {})",
                name
            )))
        }
    }

    pub fn new(s: impl Into<String>) -> Self {
        Self::try_new(s).expect("invalid set code")
    }
}

impl Display for SetCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SetCode {
    fn from(s: &str) -> Self {
        SetCode::new(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetName {
    pub code: SetCode,
    pub name: String,
}

impl SetName {
    #[allow(dead_code)]
    pub fn new(code: impl Into<SetCode>, name: impl Into<String>) -> Self {
        SetName {
            code: code.into(),
            name: name.into().to_string(),
        }
    }
}

#[cfg(test)]
#[path = "set_name_tests.rs"]
mod tests;
