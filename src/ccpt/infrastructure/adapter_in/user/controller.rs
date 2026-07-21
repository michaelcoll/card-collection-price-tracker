use crate::application::error::AppError;
use crate::domain::error::FunctionalError;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;

pub fn create_user_router() -> axum::Router<AppState> {
    axum::Router::new().route("/register", post(register))
}

#[utoipa::path(
    post,
    path = "/user/register",
    responses(
        (status = 204, description = "User registered/updated successfully"),
        (status = 400, description = "Missing username claim in token"),
        (status = 401, description = "Missing or invalid authentication token"),
    ),
    security(("bearer_auth" = [])),
    tag = "auth",
)]
pub(crate) async fn register(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<StatusCode, AppError> {
    if user.username.is_none() {
        return Err(
            FunctionalError::WrongFormat("Missing username claim in token".to_string()).into(),
        );
    }

    state.register_user_use_case.register_user(&user).await?;

    Ok(StatusCode::NO_CONTENT)
}
