use super::dto::{
    CollectionCardResponse, CollectionParams, PaginatedCollectionResponse, max_page_size,
};
use crate::application::error::AppError;
use crate::domain::collection::CollectionQuery;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::{Query, State};
use axum::routing::get;

pub fn create_collection_router() -> axum::Router<AppState> {
    axum::Router::new().route("/", get(get_collection))
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
