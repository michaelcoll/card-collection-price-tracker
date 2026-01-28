use crate::application::error::AppError;

pub trait ImportCardUseCase {
    async fn import_cards(&mut self, csv: &str) -> Result<(), AppError>;
}

pub trait CardCollectionPriceCalculationUseCase {
    async fn calculate_total_price(&mut self) -> Result<(), AppError>;
}
