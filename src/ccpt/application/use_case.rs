use crate::application::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait ImportCardUseCase: Send + Sync {
    async fn import_cards(&self, csv: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait CardCollectionPriceCalculationUseCase: Send + Sync {
    async fn calculate_total_price(&self) -> Result<(), AppError>;
}

#[async_trait]
pub trait ImportPriceUseCase: Send + Sync {
    async fn import_prices_for_current_date(&self) -> Result<(), AppError>;
}
