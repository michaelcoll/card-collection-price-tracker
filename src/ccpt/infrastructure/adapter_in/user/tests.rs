use super::controller::*;
use crate::application::error::AppError;
use crate::application::use_case::{MockRegisterUserUseCase, MockStatsUseCase};
use crate::domain::user::User;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

fn make_app_state(register_user_use_case: MockRegisterUserUseCase) -> AppState {
    AppState {
        register_user_use_case: Arc::new(register_user_use_case),
        ..AppState::for_testing(Arc::new(MockStatsUseCase::new()))
    }
}

#[tokio::test]
async fn register_returns_no_content_when_username_present() {
    let mut mock_register = MockRegisterUserUseCase::new();
    mock_register
        .expect_register_user()
        .times(1)
        .returning(|_| Box::pin(async { Ok(()) }));

    let state = make_app_state(mock_register);
    let user = User::new(
        "user_clerk123".to_string(),
        "test@example.com".to_string(),
        None,
        Some("testuser".to_string()),
    );

    let result = register(State(state), AuthenticatedUser(user)).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn register_returns_bad_request_when_username_missing() {
    let mock_register = MockRegisterUserUseCase::new();
    let state = make_app_state(mock_register);
    let user = User::new(
        "user_clerk123".to_string(),
        "test@example.com".to_string(),
        None,
        None,
    );

    let result = register(State(state), AuthenticatedUser(user)).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::WrongFormat(msg) => assert_eq!(msg, "Missing username claim in token"),
        _ => panic!("Expected WrongFormat"),
    }
}

#[tokio::test]
async fn register_propagates_use_case_error() {
    let mut mock_register = MockRegisterUserUseCase::new();
    mock_register
        .expect_register_user()
        .times(1)
        .returning(|_| Box::pin(async { Err(AppError::RepositoryError("DB error".to_string())) }));

    let state = make_app_state(mock_register);
    let user = User::new(
        "user_clerk123".to_string(),
        "test@example.com".to_string(),
        None,
        Some("testuser".to_string()),
    );

    let result = register(State(state), AuthenticatedUser(user)).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::RepositoryError(msg) => assert_eq!(msg, "DB error"),
        _ => panic!("Expected RepositoryError"),
    }
}
