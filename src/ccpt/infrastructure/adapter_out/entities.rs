#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardEntity {
    pub set_code: String,
    pub collector_number: i32,
    pub language_code: String,
    pub foil: bool,
    pub set_name: String,
    pub quantity: i32,
    /// Price in cents
    pub purchase_price: i32,
}
