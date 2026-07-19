use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreateTradeRequest {
    pub(crate) set_code: String,
    pub(crate) collector_number: String,
    pub(crate) language_code: String,
    pub(crate) foil: bool,
    pub(crate) respondent_user_id: String,
    pub(crate) quantity: u8,
}
