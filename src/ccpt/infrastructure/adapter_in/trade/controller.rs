use super::dto::CreateTradeRequest;
use crate::application::error::AppError;
use crate::domain::card::CardId;
use crate::domain::language_code::LanguageCode;
use crate::domain::user::UserId;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;

pub fn create_trade_router() -> axum::Router<AppState> {
    axum::Router::new().route("/", post(create_trade))
}

#[utoipa::path(
    post,
    path = "/trades",
    request_body = CreateTradeRequest,
    responses(
        (status = 201, description = "Trade created, or card added to (merged into) the active trade with this user"),
        (status = 400, description = "Invalid payload or respondent is the initiator"),
        (status = 401, description = "Missing or invalid token"),
        (status = 404, description = "Card not found, not owned by respondent, or respondent unknown"),
        (status = 409, description = "The active trade with this user is already fully accepted and can no longer be modified"),
    ),
    security(("bearer_auth" = [])),
    tag = "trades",
)]
pub(crate) async fn create_trade(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateTradeRequest>,
) -> Result<StatusCode, AppError> {
    let language_code = LanguageCode::try_new(&payload.language_code).map_err(|_| {
        AppError::WrongFormat(format!("Invalid language code '{}'", payload.language_code))
    })?;
    let card_id = CardId::try_new(
        payload.set_code.as_str(),
        payload.collector_number,
        language_code,
        payload.foil,
    )
    .map_err(|e| AppError::WrongFormat(String::from(e)))?;

    if payload.quantity == 0 {
        return Err(AppError::WrongFormat(
            "quantity must be at least 1".to_string(),
        ));
    }

    state
        .create_trade_use_case
        .create_trade(
            user.id,
            UserId::new(payload.respondent_user_id),
            card_id,
            payload.quantity,
        )
        .await?;

    Ok(StatusCode::CREATED)
}
