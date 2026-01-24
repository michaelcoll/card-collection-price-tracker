use crate::application::service::error::ImportError;

pub trait ImportCardUseCase {
    fn import_cards(&mut self, csv: &str) -> Result<(), ImportError>;
}
