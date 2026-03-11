use crate::application::error::AppError;
use crate::application::repository::PersistenceError;
use sqlx::Error;

pub mod card_collection_repository_adapter;
pub mod card_repository_adapter;
pub mod cardmarket_repository_adapter;
pub mod entities;
pub mod set_names_repository_adapter;

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError::RepositoryError(err.to_string())
    }
}

impl From<Error> for PersistenceError {
    fn from(_err: Error) -> Self {
        PersistenceError::DBError(_err.to_string())
    }
}
