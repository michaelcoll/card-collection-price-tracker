use crate::domain::error::CardParsingError;

#[derive(Debug, Clone)]
pub enum AppError {
    ParseError {
        line: usize,
        field: &'static str,
        value: String,
    },
    WrongFormat(String),
    CalculationFailed(String),
    RepositoryError(String),
    PriceNotFound,
    CallError(String),
    QueueError(String),
    AuthenticationError(String),
    Unauthorized,
}

impl From<AppError> for String {
    fn from(val: AppError) -> String {
        match val {
            AppError::ParseError { line, field, value } => format!(
                "Line {}: invalid {} '{}' (must be a valid value)",
                line, field, value
            ),
            AppError::WrongFormat(msg) => msg,
            AppError::CalculationFailed(msg) => msg,
            AppError::RepositoryError(msg) => msg,
            AppError::PriceNotFound => "Price not found".to_string(),
            AppError::CallError(msg) => msg,
            AppError::QueueError(msg) => msg,
            AppError::AuthenticationError(msg) => format!("Authentication error: {}", msg),
            AppError::Unauthorized => "Unauthorized".to_string(),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl std::error::Error for AppError {}

impl From<CardParsingError> for AppError {
    fn from(error: CardParsingError) -> Self {
        match error {
            CardParsingError::InvalidLanguageCode(msg) => AppError::ParseError {
                line: 0,
                field: "language_code",
                value: msg,
            },
            CardParsingError::InvalidSetCode(msg) => AppError::ParseError {
                line: 0,
                field: "set_code",
                value: msg,
            },
            CardParsingError::InvalidRarityCode(msg) => AppError::ParseError {
                line: 0,
                field: "rarity",
                value: msg,
            },
        }
    }
}

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;
