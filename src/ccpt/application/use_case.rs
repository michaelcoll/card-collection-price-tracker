use crate::application::error::AppError;
use async_trait::async_trait;

use crate::domain::collection::{CollectionQuery, PaginatedCollection};
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
pub trait EnqueueCardMarketIdUpdateUseCase: Send + Sync {
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
