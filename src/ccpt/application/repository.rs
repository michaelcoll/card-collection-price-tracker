#[cfg(test)]
use mockall::automock;

use crate::domain::card::Card;
use crate::domain::price::Price;
use crate::domain::set_name::{SetCode, SetName};

#[derive(Debug, PartialEq, Eq)]
pub enum PersistenceError {
    SaveError(String),
}

impl From<PersistenceError> for String {
    fn from(val: PersistenceError) -> String {
        match val {
            PersistenceError::SaveError(msg) => msg,
        }
    }
}

#[cfg_attr(test, automock)]
pub trait CardRepository {
    fn get_all(&self) -> Result<Vec<Card>, PersistenceError>;
    fn save(&mut self, card: Card) -> Result<(), PersistenceError>;
    fn delete_all(&mut self) -> Result<(), PersistenceError>;
}

#[cfg_attr(test, automock)]
pub trait SetNameRepository {
    fn exists_by_code(&self, code: SetCode) -> Result<bool, PersistenceError>;
    fn save(&mut self, set: SetName) -> Result<(), PersistenceError>;
}

#[cfg_attr(test, automock)]
pub trait CardCollectionRepository {
    fn save(&mut self, price: Price) -> Result<(), PersistenceError>;
}
