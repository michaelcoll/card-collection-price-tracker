use crate::domain::card::Card;
use crate::domain::price::PriceGuide;
use crate::domain::set_name::{SetCode, SetName};
use async_trait::async_trait;

use crate::application::error::AppError;
use crate::domain::user::User;
#[cfg(test)]
use mockall::automock;

#[derive(Debug, PartialEq, Eq)]
pub enum PersistenceError {
    DBError(String),
}

impl From<PersistenceError> for String {
    fn from(val: PersistenceError) -> String {
        match val {
            PersistenceError::DBError(msg) => msg,
        }
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardRepository: Send + Sync {
    async fn get_all(&self, user: User) -> Result<Vec<Card>, AppError>;
    async fn save(&self, user: User, card: Card) -> Result<(), AppError>;
    async fn delete_all(&self, user: User) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait SetNameRepository: Send + Sync {
    async fn exists_by_code(&self, code: SetCode) -> Result<bool, AppError>;
    async fn save(&self, set: SetName) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardCollectionRepository: Send + Sync {
    async fn save(&self, price: PriceGuide) -> Result<(), AppError>;
}
