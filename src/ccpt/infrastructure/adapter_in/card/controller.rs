use super::dto::{
    CardOfferResponse, CardOffersParams, PaginatedCardOffersResponse, PriceHistoryEntryResponse,
    PriceHistoryParams,
};
use crate::application::error::AppError;
use crate::domain::card::CardId;
use crate::domain::language_code::LanguageCode;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use uuid::Uuid;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/card-info", post(get_card_info))
        .route("/{scryfall_id}/price-history", get(get_card_price_history))
        .route("/offers", get(get_card_offers))
}

#[utoipa::path(
    post,
    path = "/card/card-info",
    responses(
        (status = 200, description = "Card info from EDHRec"),
        (status = 401, description = "Missing or invalid token"),
    ),
    security(("bearer_auth" = [])),
    tag = "card",
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
    path = "/card/{scryfall_id}/price-history",
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
    tag = "card",
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

#[utoipa::path(
    get,
    path = "/card/offers",
    params(
        ("set_code" = String, Query, description = "Card's set code"),
        ("collector_number" = String, Query, description = "Card's collector number"),
        ("language_code" = String, Query, description = "Card's language code"),
        ("foil" = bool, Query, description = "Whether the card is foil"),
        ("sort_by" = Option<super::dto::CardOffersSortByParam>, Query, description = "Sort field (only selling_price supported for now)"),
        ("page" = Option<u32>, Query, description = "Page number (starts at 0, max 10)"),
        ("page_size" = Option<u32>, Query, description = "Items per page (1 to 100)"),
    ),
    responses(
        (status = 200, description = "Paginated list of sale offers for this card", body = PaginatedCardOffersResponse),
        (status = 400, description = "Invalid or missing query params"),
        (status = 401, description = "Missing or invalid token"),
        (status = 404, description = "No card found for this CardId"),
    ),
    security(("bearer_auth" = [])),
    tag = "card",
)]
pub(crate) async fn get_card_offers(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Query(params): Query<CardOffersParams>,
) -> Result<axum::Json<PaginatedCardOffersResponse>, AppError> {
    let language_code = LanguageCode::try_new(&params.language_code)?;
    let card_id = CardId::try_new(
        params.set_code.as_str(),
        params.collector_number,
        language_code,
        params.foil,
    )?;
    let page_size = params.page_size.clamp(1, state.max_page_size);
    let page = params.page.min(state.max_page_number);

    let result = state
        .get_card_offers_use_case
        .get_card_offers(&user.id, card_id, params.sort_by.into(), page, page_size)
        .await?;

    Ok(axum::Json(PaginatedCardOffersResponse {
        items: result
            .items
            .into_iter()
            .map(CardOfferResponse::from)
            .collect(),
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}
