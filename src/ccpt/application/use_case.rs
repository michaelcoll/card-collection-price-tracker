use crate::application::error::AppError;
use async_trait::async_trait;

use crate::domain::collection::{CollectionQuery, PaginatedCollection};
use crate::domain::collection_stats::CollectionStats;
use crate::domain::price::PriceHistoryEntry;
use crate::domain::stats::Stats;
use crate::domain::user::User;
#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait ImportCardUseCase: Send + Sync {
    async fn import_cards(&self, csv: &str, user: User) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait RegisterUserUseCase: Send + Sync {
    async fn register_user(&self, user: &User) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait EnqueueCardMarketIdUpdateUseCase: Send + Sync {
    async fn enqueue_pending_updates(&self) -> Result<usize, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait EnqueueGathererIdUpdateUseCase: Send + Sync {
    async fn enqueue_pending_updates(&self) -> Result<usize, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CardCollectionPriceCalculationUseCase: Send + Sync {
    async fn calculate_total_price(&self) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait ImportPriceUseCase: Send + Sync {
    async fn import_prices_for_current_date(&self) -> Result<(), AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait StatsUseCase: Send + Sync {
    async fn get_stats(&self) -> Result<Stats, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait GetCollectionUseCase: Send + Sync {
    async fn get_collection(
        &self,
        user_id: &str,
        query: CollectionQuery,
    ) -> Result<PaginatedCollection, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait GetCollectionPriceHistoryUseCase: Send + Sync {
    async fn get_collection_price_history(
        &self,
        user_id: &str,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> Result<Vec<PriceHistoryEntry>, AppError>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait GetCollectionStatsUseCase: Send + Sync {
    async fn get_collection_stats(&self, user_id: &str) -> Result<CollectionStats, AppError>;
}
