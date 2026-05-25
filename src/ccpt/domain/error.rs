#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardParsingError {
    InvalidLanguageCode(String),
    InvalidSetCode(String),
    InvalidRarityCode(String),
}

impl From<CardParsingError> for String {
    fn from(val: CardParsingError) -> String {
        match val {
            CardParsingError::InvalidLanguageCode(msg) => msg,
            CardParsingError::InvalidSetCode(msg) => msg,
            CardParsingError::InvalidRarityCode(msg) => msg,
        }
    }
}
