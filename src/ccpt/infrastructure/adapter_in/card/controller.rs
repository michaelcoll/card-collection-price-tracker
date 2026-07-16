use super::dto::{
    CollectionCardResponse, CollectionParams, CollectionStatsResponse, MessageResponse,
    PaginatedCollectionResponse, PriceHistoryEntryResponse, PriceHistoryParams, max_page_size,
};
use crate::application::error::AppError;
use crate::domain::collection::CollectionQuery;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::body::to_bytes;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use uuid::Uuid;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_collection))
        .route("/import", post(import_cards))
        .route("/card-info", post(get_card_info))
        .route("/price-history", get(get_collection_price_history))
        .route("/stats", get(get_collection_stats))
        .route("/{scryfall_id}/price-history", get(get_card_price_history))
}

#[utoipa::path(
    get,
    path = "/cards/",
    params(
        ("page" = Option<u32>, Query, description = "Page number (starts at 0)"),
        ("page_size" = Option<u32>, Query, description = "Items per page (max 100)"),
        ("sort_by" = Option<super::dto::SortByParam>, Query, description = "Sort field"),
        ("sort_dir" = Option<super::dto::SortDirParam>, Query, description = "Sort direction"),
        ("q" = Option<String>, Query, description = "Fuzzy search on card name or set"),
        ("rarity" = Option<String>, Query, description = "Comma-separated rarity codes (C, U, R, M)"),
        ("sets" = Option<String>, Query, description = "Comma-separated set codes"),
        ("price_min" = Option<u32>, Query, description = "Minimum trend price in cents"),
        ("price_max" = Option<u32>, Query, description = "Maximum trend price in cents"),
        ("owned" = Option<bool>, Query, description = "Restrict to cards owned by the authenticated user (default: false — full catalog)"),
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

    let rarity = params
        .rarity
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            crate::domain::rarity_code::RarityCode::try_new(s)
                .map_err(|_| AppError::WrongFormat(format!("Invalid rarity code '{}'", s)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let sets = params
        .sets
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_uppercase)
        .collect::<Vec<_>>();

    let query = CollectionQuery {
        page: params.page,
        page_size,
        sort_by: params.sort_by.into(),
        sort_dir: params.sort_dir.into(),
        search_query: params.q,
        rarity,
        sets,
        price_min: params.price_min,
        price_max: params.price_max,
        owned: params.owned,
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

    tracing::info!("Importing cards for user: {} ({})", user.email, user.id);

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
    tracing::info!("Getting card info for user: {} ({})", user.email, user.id);

    state
        .edh_rec_caller_adapter
        .get_card_info("Sol Ring".to_string())
        .await
        .expect("panic message");

    Ok("card Info".to_string())
}

#[utoipa::path(
    get,
    path = "/cards/stats",
    responses(
        (status = 200, description = "Collection stats for the authenticated user", body = CollectionStatsResponse),
        (status = 401, description = "Missing or invalid token"),
    ),
    security(("bearer_auth" = [])),
    tag = "cards",
)]
pub(crate) async fn get_collection_stats(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<axum::Json<CollectionStatsResponse>, AppError> {
    let stats = state
        .get_collection_stats_use_case
        .get_collection_stats(&user.id)
        .await?;
    Ok(axum::Json(CollectionStatsResponse::from(stats)))
}

#[utoipa::path(
    get,
    path = "/cards/price-history",
    params(
        ("start_date" = Option<String>, Query, description = "Start date (ISO 8601: YYYY-MM-DD, inclusive). Defaults to end_date minus 30 days"),
        ("end_date" = Option<String>, Query, description = "End date (ISO 8601: YYYY-MM-DD, inclusive). Defaults to today"),
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

#[utoipa::path(
    get,
    path = "/cards/{scryfall_id}/price-history",
    params(
        ("scryfall_id" = Uuid, Path, description = "Card's Scryfall identifier"),
        ("start_date" = Option<String>, Query, description = "Start date (ISO 8601: YYYY-MM-DD, inclusive). Defaults to end_date minus 30 days"),
        ("end_date" = Option<String>, Query, description = "End date (ISO 8601: YYYY-MM-DD, inclusive). Defaults to today"),
    ),
    responses(
        (status = 200, description = "Card price history", body = Vec<PriceHistoryEntryResponse>),
        (status = 400, description = "Invalid date range (start_date > end_date)"),
        (status = 401, description = "Missing or invalid token"),
        (status = 404, description = "No card found for this scryfall_id"),
    ),
    security(("bearer_auth" = [])),
    tag = "cards",
)]
pub(crate) async fn get_card_price_history(
    AuthenticatedUser(_user): AuthenticatedUser,
    State(state): State<AppState>,
    Path(scryfall_id): Path<Uuid>,
    Query(params): Query<PriceHistoryParams>,
) -> Result<axum::Json<Vec<PriceHistoryEntryResponse>>, AppError> {
    let entries = state
        .get_card_price_history_use_case
        .get_card_price_history(scryfall_id, params.start_date, params.end_date)
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
