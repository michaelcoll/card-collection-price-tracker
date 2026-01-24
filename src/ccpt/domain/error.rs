#[derive(Debug, PartialEq, Eq)]
pub enum CardParsingError {
    InvalidLanguageCode(String),
    InvalidSetCode(String),
}

impl From<CardParsingError> for String {
    fn from(val: CardParsingError) -> String {
        match val {
            CardParsingError::InvalidLanguageCode(msg) => msg,
            CardParsingError::InvalidSetCode(msg) => msg,
        }
    }
}
