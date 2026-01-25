use crate::application::service::error::{CalculationError, ImportError};

pub trait ImportCardUseCase {
    fn import_cards(&mut self, csv: &str) -> Result<(), ImportError>;
}

pub trait CardCollectionPriceCalculationUseCase {
    fn calculate_total_price(&mut self) -> Result<(), CalculationError>;
}
