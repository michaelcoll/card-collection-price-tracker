use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

// --- Price history ---
/// Shared by `/collection/price-history` and `/cards/{scryfall_id}/price-history`; both dates are
/// optional and defaulted by the use case (last 30 days when absent).
#[derive(Deserialize, TS)]
#[ts(export, export_to = "PriceHistoryParams.ts")]
pub(crate) struct PriceHistoryParams {
    /// ISO 8601 date string (YYYY-MM-DD)
    #[ts(optional, type = "string")]
    pub(crate) start_date: Option<NaiveDate>,
    /// ISO 8601 date string (YYYY-MM-DD)
    #[ts(optional, type = "string")]
    pub(crate) end_date: Option<NaiveDate>,
}

#[derive(Serialize, Debug, TS, ToSchema)]
#[serde(rename = "PriceHistoryEntry")]
#[ts(export, export_to = "PriceHistoryEntry.ts")]
pub struct PriceHistoryEntryResponse {
    /// ISO 8601 date string (YYYY-MM-DD)
    pub date: String,
    pub low: i64,
    pub trend: i64,
    pub avg: i64,
}
