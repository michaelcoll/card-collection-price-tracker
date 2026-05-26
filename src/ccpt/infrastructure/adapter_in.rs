use crate::application::error::AppError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub mod auth_extractor;
pub mod card_controller;
pub mod maintenance_controller;
pub mod openapi;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::ParseError { .. } => (StatusCode::BAD_REQUEST, String::from(self.clone())),
            AppError::WrongFormat(_) => (StatusCode::BAD_REQUEST, String::from(self.clone())),
            AppError::PriceNotFound => (StatusCode::NOT_FOUND, String::from(self.clone())),
            AppError::CallError(_) => (StatusCode::BAD_GATEWAY, String::from(self.clone())),
            AppError::AuthenticationError(_) => {
                (StatusCode::UNAUTHORIZED, String::from(self.clone()))
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, String::from(self.clone())),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from(self.clone()),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
#[path = "adapter_in_tests.rs"]
mod tests;
