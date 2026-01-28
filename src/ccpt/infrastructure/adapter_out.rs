use crate::application::error::AppError;
use sqlx::Error;

pub mod card_entity_mapper;
pub mod card_repository_adapter;
pub mod entities;

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError::RepositoryError(err.to_string())
    }
}
