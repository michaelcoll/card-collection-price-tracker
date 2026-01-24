use crate::domain::language_code::LanguageCode;
use crate::domain::set_name::SetName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub set_name: SetName,
    pub collector_number: u16,
    pub language_code: LanguageCode,
    pub name: String,
    pub foil: bool,
    pub quantity: u8,
    /// Price in cents
    pub purchase_price: u32,
}
