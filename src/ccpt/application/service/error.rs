use crate::application::repository::PersistenceError;
use crate::domain::error::CardParsingError;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

#[derive(Debug)]
pub enum ImportError {
    ParseError(String),
    WrongFormat(String),
}

impl From<ParseIntError> for ImportError {
    fn from(err: ParseIntError) -> Self {
        ImportError::ParseError(err.to_string())
    }
}

impl From<ParseFloatError> for ImportError {
    fn from(err: ParseFloatError) -> Self {
        ImportError::ParseError(err.to_string())
    }
}

impl From<ParseBoolError> for ImportError {
    fn from(err: ParseBoolError) -> Self {
        ImportError::ParseError(err.to_string())
    }
}

impl From<CardParsingError> for ImportError {
    fn from(err: CardParsingError) -> Self {
        ImportError::ParseError(err.into())
    }
}

impl From<PersistenceError> for ImportError {
    fn from(err: PersistenceError) -> Self {
        ImportError::ParseError(err.into())
    }
}
