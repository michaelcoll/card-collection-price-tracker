use crate::domain::user::User;
use crate::infrastructure::AppState;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
};

#[derive(Debug)]
pub struct AuthenticatedUser(pub User);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = (StatusCode, String);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let auth_header = parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Missing Authorization header".to_string(),
                    )
                })?;

            let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header format".to_string(),
                )
            })?;

            let user = state
                .auth_service
                .validate_token(token)
                .await
                .map_err(|e| {
                    (
                        StatusCode::UNAUTHORIZED,
                        format!("Authentication failed: {}", e),
                    )
                })?;

            Ok(AuthenticatedUser(user))
        }
    }
}

#[cfg(test)]
#[path = "auth_extractor_tests.rs"]
mod tests;
