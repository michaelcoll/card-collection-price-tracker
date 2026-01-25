use crate::application::service::error::{CalculationError, ImportError};

pub trait ImportCardUseCase {
    async fn import_cards(&mut self, csv: &str) -> Result<(), ImportError>;
}

pub trait CardCollectionPriceCalculationUseCase {
    async fn calculate_total_price(&mut self) -> Result<(), CalculationError>;
}
