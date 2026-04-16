use crate::application::error::AppError;
use crate::domain::card::{Card, CardId};
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
    #[allow(dead_code)]
    async fn get_all(&self, user: User) -> Result<Vec<Card>, AppError>;
    async fn get_all_without_cardmarket_id(&self) -> Result<Vec<(CardId, uuid::Uuid)>, AppError>;
    async fn save(&self, user: User, card: Card) -> Result<(), AppError>;
    async fn update_cardmarket_id(
        &self,
        id: CardId,
        cardmarket_id: Option<u32>,
    ) -> Result<(), AppError>;
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
pub trait CollectionPriceHistoryRepository: Send + Sync {
    async fn get_date_and_user_to_update(&self) -> Result<Vec<(NaiveDate, User)>, AppError>;
    async fn update_for_date_and_user(&self, date: NaiveDate, user: User) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardMarketPriceRepository: Send + Sync {
    async fn save(
        &self,
        date: NaiveDate,
        price_guides: Vec<FullPriceGuide>,
    ) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardPricesViewRepository: Send + Sync {
    async fn refresh(&self) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait StatsRepository: Send + Sync {
    async fn get_card_number(&self) -> Result<u32, AppError>;
    async fn get_card_price_number(&self) -> Result<u32, AppError>;
    async fn get_db_size(&self) -> Result<u16, AppError>;
}
