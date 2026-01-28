use crate::domain::card::Card;
use crate::domain::price::Price;
use crate::domain::set_name::{SetCode, SetName};

use crate::application::error::AppError;
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

#[cfg_attr(test, automock)]
pub trait CardRepository {
    #[allow(dead_code)]
    async fn get_all(&self) -> Result<Vec<Card>, AppError>;
    #[allow(dead_code)]
    async fn save(&mut self, card: Card) -> Result<(), AppError>;
    #[allow(dead_code)]
    async fn delete_all(&mut self) -> Result<(), AppError>;
}

#[cfg_attr(test, automock)]
pub trait SetNameRepository {
    #[allow(dead_code)]
    async fn exists_by_code(&self, code: SetCode) -> Result<bool, AppError>;
    #[allow(dead_code)]
    async fn save(&mut self, set: SetName) -> Result<(), AppError>;
}

#[cfg_attr(test, automock)]
pub trait CardCollectionRepository {
    #[allow(dead_code)]
    async fn save(&mut self, price: Price) -> Result<(), AppError>;
}
