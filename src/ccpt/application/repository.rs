use crate::application::error::AppError;
use crate::domain::card::Card;
use crate::domain::price::FullPriceGuide;
use crate::domain::set_name::{SetCode, SetName};
use crate::domain::user::User;
use async_trait::async_trait;
use chrono::NaiveDate;
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
    async fn get_all_without_cardmarket_id(&self) -> Result<Vec<Card>, AppError>;
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
    async fn get_date_and_user_to_update(&self) -> Result<Vec<(NaiveDate, User)>, AppError>;
    async fn update_for_date_and_user(&self, date: NaiveDate, user: User) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardMarketRepository: Send + Sync {
    async fn save(
        &self,
        date: NaiveDate,
        price_guides: Vec<FullPriceGuide>,
    ) -> Result<(), AppError>;
}
