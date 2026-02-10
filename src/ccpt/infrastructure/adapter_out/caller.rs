use crate::application::error::AppError;

pub mod cardmarket_caller_adapter;
mod dto;

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::CallError(value.to_string())
    }
}
