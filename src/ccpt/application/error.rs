use crate::application::repository::PersistenceError;
use crate::domain::error::CardParsingError;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

#[derive(Debug)]
pub enum AppError {
    ParseError(),
    WrongFormat(String),
    CalculationFailed(String),
    RepositoryError(String),
    PriceNotFound,
    CallError(String),
}

impl From<ParseIntError> for AppError {
    fn from(_err: ParseIntError) -> Self {
        AppError::ParseError()
    }
}

impl From<ParseFloatError> for AppError {
    fn from(_err: ParseFloatError) -> Self {
        AppError::ParseError()
    }
}

impl From<ParseBoolError> for AppError {
    fn from(_err: ParseBoolError) -> Self {
        AppError::ParseError()
    }
}

impl From<CardParsingError> for AppError {
    fn from(err: CardParsingError) -> Self {
        AppError::WrongFormat(err.into())
    }
}

impl From<PersistenceError> for AppError {
    fn from(err: PersistenceError) -> Self {
        AppError::RepositoryError(err.into())
    }
}

impl From<AppError> for String {
    fn from(val: AppError) -> String {
        match val {
            AppError::ParseError() => "Parse Error".to_string(),
            AppError::WrongFormat(msg) => msg,
            AppError::CalculationFailed(msg) => msg,
            AppError::RepositoryError(msg) => msg,
            AppError::PriceNotFound => "Price not found".to_string(),
            AppError::CallError(msg) => msg,
        }
    }
}
