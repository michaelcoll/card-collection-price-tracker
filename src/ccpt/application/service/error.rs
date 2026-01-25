use crate::application::caller::CallerError;
use crate::application::repository::PersistenceError;
use crate::domain::error::CardParsingError;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

#[derive(Debug)]
pub enum ImportError {
    ParseError(),
    WrongFormat(String),
}

impl From<ParseIntError> for ImportError {
    fn from(_err: ParseIntError) -> Self {
        ImportError::ParseError()
    }
}

impl From<ParseFloatError> for ImportError {
    fn from(_err: ParseFloatError) -> Self {
        ImportError::ParseError()
    }
}

impl From<ParseBoolError> for ImportError {
    fn from(_err: ParseBoolError) -> Self {
        ImportError::ParseError()
    }
}

impl From<CardParsingError> for ImportError {
    fn from(err: CardParsingError) -> Self {
        ImportError::WrongFormat(err.into())
    }
}

impl From<PersistenceError> for ImportError {
    fn from(_err: PersistenceError) -> Self {
        ImportError::ParseError()
    }
}

#[derive(Debug)]
pub enum CalculationError {
    CalculationFailed(String),
}

impl From<CalculationError> for String {
    fn from(val: CalculationError) -> String {
        match val {
            CalculationError::CalculationFailed(msg) => msg,
        }
    }
}

impl From<CallerError> for CalculationError {
    fn from(err: CallerError) -> Self {
        CalculationError::CalculationFailed(err.into())
    }
}

impl From<PersistenceError> for CalculationError {
    fn from(err: PersistenceError) -> Self {
        CalculationError::CalculationFailed(err.into())
    }
}

impl From<CardParsingError> for CalculationError {
    fn from(err: CardParsingError) -> Self {
        CalculationError::CalculationFailed(err.into())
    }
}
