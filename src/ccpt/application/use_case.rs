use crate::application::error::AppError;

#[allow(dead_code)]
pub trait ImportCardUseCase {
    async fn import_cards(&mut self, csv: &str) -> Result<(), AppError>;
}

#[allow(dead_code)]
pub trait CardCollectionPriceCalculationUseCase {
    async fn calculate_total_price(&mut self) -> Result<(), AppError>;
}
