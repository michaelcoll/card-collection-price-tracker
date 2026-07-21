use super::dto::{CollectionStatsResponse, PriceHistoryEntryResponse, PriceHistoryParams};
use crate::application::error::AppError;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use uuid::Uuid;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/card-info", post(get_card_info))
        .route("/price-history", get(get_collection_price_history))
        .route("/stats", get(get_collection_stats))
        .route("/{scryfall_id}/price-history", get(get_card_price_history))
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
    tracing::info!("Getting card info for user: {}", user.id);

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
