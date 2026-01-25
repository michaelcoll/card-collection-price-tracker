use crate::domain::card::Card;
use crate::domain::price::Price;
use crate::domain::set_name::{SetCode, SetName};

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
    async fn get_all(&self) -> Result<Vec<Card>, PersistenceError>;
    async fn save(&mut self, card: Card) -> Result<(), PersistenceError>;
    async fn delete_all(&mut self) -> Result<(), PersistenceError>;
}

#[cfg_attr(test, automock)]
pub trait SetNameRepository {
    async fn exists_by_code(&self, code: SetCode) -> Result<bool, PersistenceError>;
    async fn save(&mut self, set: SetName) -> Result<(), PersistenceError>;
}

#[cfg_attr(test, automock)]
pub trait CardCollectionRepository {
    async fn save(&mut self, price: Price) -> Result<(), PersistenceError>;
}
