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
mod tests {
    use super::*;

    #[test]
    fn parse_error_returns_bad_request_status() {
        let error = AppError::ParseError {
            line: 5,
            field: "quantity",
            value: "invalid".to_string(),
        };
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn parse_error_with_different_line_number_returns_bad_request() {
        let error = AppError::ParseError {
            line: 42,
            field: "foil",
            value: "maybe".to_string(),
        };
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn wrong_format_returns_bad_request_status() {
        let error = AppError::WrongFormat("CSV header missing".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn wrong_format_with_empty_message_returns_bad_request() {
        let error = AppError::WrongFormat(String::new());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn price_not_found_returns_not_found_status() {
        let error = AppError::PriceNotFound;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn call_error_returns_bad_gateway_status() {
        let error = AppError::CallError("External API timeout".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    }

    #[test]
    fn call_error_with_network_failure_returns_bad_gateway() {
        let error = AppError::CallError("Connection refused".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    }

    #[test]
    fn calculation_failed_returns_internal_server_error_status() {
        let error = AppError::CalculationFailed("Division by zero".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn repository_error_returns_internal_server_error_status() {
        let error = AppError::RepositoryError("Database connection lost".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn queue_error_returns_internal_server_error_status() {
        let error = AppError::QueueError("Queue overflow".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
