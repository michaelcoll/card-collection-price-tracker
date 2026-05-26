use crate::application::error::AppError;
use crate::domain::card::Card;
use crate::domain::collection::{CollectionQuery, CollectionSortField, SortDirection};
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::body::to_bytes;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_collection))
        .route("/import", post(import_cards))
        .route("/card-info", post(get_card_info))
        .route("/price-history", get(get_collection_price_history))
}

// --- Query params ---
#[derive(Deserialize, Default, TS, ToSchema)]
#[serde(rename = "SortBy", rename_all = "snake_case")]
#[ts(export, export_to = "SortBy.ts")]
pub enum SortByParam {
    Avg,
    #[default]
    Trend,
    SetCode,
    LanguageCode,
}

#[derive(Deserialize, Default, TS, ToSchema)]
#[serde(rename = "SortDir", rename_all = "snake_case")]
#[ts(export, export_to = "SortDir.ts")]
pub enum SortDirParam {
    Asc,
    #[default]
    Desc,
}

fn default_page_size() -> u32 {
    20
}

fn max_page_size() -> u32 {
    100
}

#[derive(Deserialize)]
pub(crate) struct CollectionParams {
    #[serde(default)]
    pub(crate) page: u32,
    #[serde(default = "default_page_size")]
    pub(crate) page_size: u32,
    #[serde(default)]
    pub(crate) sort_by: SortByParam,
    #[serde(default)]
    pub(crate) sort_dir: SortDirParam,
    pub(crate) q: Option<String>,
}

// --- Réponses ---
#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "PriceGuide")]
#[ts(export, export_to = "PriceGuide.ts")]
pub struct PriceGuideResponse {
    pub low: Option<u32>,
    pub avg: Option<u32>,
    pub trend: Option<u32>,
    pub avg1: Option<u32>,
    pub avg7: Option<u32>,
    pub avg30: Option<u32>,
}

#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "CollectionCard")]
#[ts(export, export_to = "CollectionCard.ts")]
pub struct CollectionCardResponse {
    pub set_code: String,
    pub collector_number: String,
    pub language_code: String,
    pub foil: bool,
    pub name: String,
    pub rarity_code: String,
    pub scryfall_id: String,
    pub quantity: u8,
    pub purchase_price: u32,
    pub price_guide: Option<PriceGuideResponse>,
}

#[derive(Serialize, Debug, TS, ToSchema)]
#[serde(rename = "Message")]
#[ts(export, export_to = "Message.ts")]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "PaginatedCollection")]
#[ts(export, export_to = "PaginatedCollection.ts")]
pub struct PaginatedCollectionResponse {
    pub items: Vec<CollectionCardResponse>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

impl From<Card> for CollectionCardResponse {
    fn from(c: Card) -> Self {
        Self {
            set_code: c.id.set_code.to_string(),
            collector_number: c.id.collector_number,
            language_code: c.id.language_code.to_string(),
            foil: c.id.foil,
            name: c.name,
            rarity_code: c.rarity_code.to_string(),
            scryfall_id: c.scryfall_id.to_string(),
            quantity: c.quantity,
            purchase_price: c.purchase_price,
            price_guide: c.price_guide.map(|pg| PriceGuideResponse {
                low: pg.low.value,
                avg: pg.avg.value,
                trend: pg.trend.value,
                avg1: pg.avg1.value,
                avg7: pg.avg7.value,
                avg30: pg.avg30.value,
            }),
        }
    }
}

// --- Handler ---
#[utoipa::path(
    get,
    path = "/cards/",
    params(
        ("page" = Option<u32>, Query, description = "Page number (starts at 0)"),
        ("page_size" = Option<u32>, Query, description = "Items per page (max 100)"),
        ("sort_by" = Option<SortByParam>, Query, description = "Sort field"),
        ("sort_dir" = Option<SortDirParam>, Query, description = "Sort direction"),
        ("q" = Option<String>, Query, description = "Fuzzy search on card name or set"),
    ),
    responses(
        (status = 200, description = "Paginated card collection", body = PaginatedCollectionResponse),
        (status = 401, description = "Missing or invalid token"),
    ),
    security(("bearer_auth" = [])),
    tag = "cards",
)]
pub(crate) async fn get_collection(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Query(params): Query<CollectionParams>,
) -> Result<axum::Json<PaginatedCollectionResponse>, AppError> {
    let page_size = params.page_size.min(max_page_size());

    let query = CollectionQuery {
        page: params.page,
        page_size,
        sort_by: match params.sort_by {
            SortByParam::Avg => CollectionSortField::Avg,
            SortByParam::Trend => CollectionSortField::Trend,
            SortByParam::SetCode => CollectionSortField::SetCode,
            SortByParam::LanguageCode => CollectionSortField::LanguageCode,
        },
        sort_dir: match params.sort_dir {
            SortDirParam::Asc => SortDirection::Asc,
            SortDirParam::Desc => SortDirection::Desc,
        },
        search_query: params.q,
    };

    let result = state
        .get_collection_use_case
        .get_collection(&user.id, query)
        .await?;

    Ok(axum::Json(PaginatedCollectionResponse {
        items: result
            .items
            .into_iter()
            .map(CollectionCardResponse::from)
            .collect(),
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}

#[utoipa::path(
    post,
    path = "/cards/import",
    request_body(
        content = String,
        content_type = "text/plain",
        description = "ManaBox CSV content (max 10 MB)",
    ),
    responses(
        (status = 200, description = "Import successful", body = MessageResponse),
        (status = 400, description = "Invalid body (non UTF-8, ...)"),
        (status = 401, description = "Missing or invalid token"),
    ),
    security(("bearer_auth" = [])),
    tag = "cards",
)]
pub(crate) async fn import_cards(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    body: axum::body::Body,
) -> Result<axum::Json<MessageResponse>, AppError> {
    let bytes = to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|e| AppError::WrongFormat(format!("Failed to read body: {}", e)))?;

    let csv = String::from_utf8(bytes.to_vec())
        .map_err(|_| AppError::WrongFormat("Body is not valid UTF-8".to_string()))?;

    // L'user.id est maintenant disponible pour associer les données importées à l'utilisateur
    println!("Importing cards for user: {} ({})", user.email, user.id);

    state
        .import_card_use_case
        .clone()
        .import_cards(&csv, user)
        .await?;

    Ok(axum::Json(MessageResponse {
        message: "Cards imported successfully".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/cards/card-info",
    responses(
        (status = 200, description = "Card info from EDHRec"),
        (status = 401, description = "Missing or invalid token"),
    ),
    security(("bearer_auth" = [])),
    tag = "cards",
)]
pub(crate) async fn get_card_info(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    println!("Getting card info for user: {} ({})", user.email, user.id);

    state
        .edh_rec_caller_adapter
        .get_card_info("Sol Ring".to_string())
        .await
        .expect("panic message");

    Ok("card Info".to_string())
}

// --- Price history ---
#[derive(Deserialize)]
pub(crate) struct PriceHistoryParams {
    start_date: NaiveDate,
    end_date: NaiveDate,
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

#[utoipa::path(
    get,
    path = "/cards/price-history",
    params(
        ("start_date" = String, Query, description = "Start date (ISO 8601: YYYY-MM-DD, inclusive)"),
        ("end_date" = String, Query, description = "End date (ISO 8601: YYYY-MM-DD, inclusive)"),
    ),
    responses(
        (status = 200, description = "Collection price history", body = Vec<PriceHistoryEntryResponse>),
        (status = 400, description = "Invalid date range (start_date > end_date)"),
        (status = 401, description = "Missing or invalid token"),
    ),
    security(("bearer_auth" = [])),
    tag = "cards",
)]
pub(crate) async fn get_collection_price_history(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Query(params): Query<PriceHistoryParams>,
) -> Result<axum::Json<Vec<PriceHistoryEntryResponse>>, AppError> {
    let entries = state
        .get_collection_price_history_use_case
        .get_collection_price_history(&user.id, params.start_date, params.end_date)
        .await?;

    Ok(axum::Json(
        entries
            .into_iter()
            .map(|e| PriceHistoryEntryResponse {
                date: e.date.to_string(),
                low: e.price_guide.low.value.unwrap_or(0) as i64,
                trend: e.price_guide.trend.value.unwrap_or(0) as i64,
                avg: e.price_guide.avg.value.unwrap_or(0) as i64,
            })
            .collect(),
    ))
}

#[cfg(test)]
#[path = "card_controller_tests.rs"]
mod tests;
