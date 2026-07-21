use crate::domain::collection_stats::CollectionStats;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Serialize, Debug, TS, ToSchema)]
#[serde(rename = "Message")]
#[ts(export, export_to = "Message.ts")]
pub struct MessageResponse {
    pub message: String,
}

// --- Collection stats ---
#[derive(Serialize, Debug, TS, ToSchema)]
#[serde(rename = "SetInfo")]
#[ts(export, export_to = "SetInfo.ts")]
pub struct SetInfoResponse {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Debug, TS, ToSchema)]
#[serde(rename = "CollectionStats")]
#[ts(export, export_to = "CollectionStats.ts")]
pub struct CollectionStatsResponse {
    pub total_cards: u64,
    pub unique_cards: u64,
    pub price_trend_min: Option<u32>,
    pub price_trend_max: Option<u32>,
    pub sets: Vec<SetInfoResponse>,
}

impl From<CollectionStats> for CollectionStatsResponse {
    fn from(s: CollectionStats) -> Self {
        Self {
            total_cards: s.total_cards,
            unique_cards: s.unique_cards,
            price_trend_min: s.price_trend_min.value,
            price_trend_max: s.price_trend_max.value,
            sets: s
                .sets
                .into_iter()
                .map(|sn| SetInfoResponse {
                    code: sn.code.to_string(),
                    name: sn.name,
                })
                .collect(),
        }
    }
}

// --- Price history ---
/// Shared by `/cards/price-history` (both dates required, enforced by the handler) and
/// `/cards/{scryfall_id}/price-history` (both optional, defaulted by the use case).
#[derive(Deserialize)]
pub(crate) struct PriceHistoryParams {
    pub(crate) start_date: Option<NaiveDate>,
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
