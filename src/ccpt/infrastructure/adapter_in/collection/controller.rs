use super::dto::{
    CollectionCardResponse, CollectionParams, MessageResponse, PaginatedCollectionResponse,
    max_page_size,
};
use crate::application::error::AppError;
use crate::domain::collection::CollectionQuery;
use crate::domain::error::FunctionalError;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::body::to_bytes;
use axum::extract::{Query, State};
use axum::routing::{get, post};

pub fn create_collection_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_collection))
        .route("/import", post(import_cards))
}

#[utoipa::path(
    post,
    path = "/collection/import",
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
    tag = "collection",
)]
pub(crate) async fn import_cards(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    body: axum::body::Body,
) -> Result<axum::Json<MessageResponse>, AppError> {
    let bytes = to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|e| FunctionalError::WrongFormat(format!("Failed to read body: {}", e)))?;

    let csv = String::from_utf8(bytes.to_vec())
        .map_err(|_| FunctionalError::WrongFormat("Body is not valid UTF-8".to_string()))?;

    tracing::info!("Importing cards for user: {}", user.id);

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
    get,
    path = "/collection",
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
    tag = "collection",
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
        .map(|s| crate::domain::rarity_code::RarityCode::try_new(s).map_err(AppError::from))
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
