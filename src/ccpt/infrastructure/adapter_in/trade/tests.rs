use super::controller::*;
use super::dto::CreateTradeRequest;
use crate::application::error::AppError;
use crate::application::use_case::{MockCreateTradeUseCase, MockStatsUseCase};
use crate::domain::trade::TradeId;
use crate::domain::user::User;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

fn make_app_state(create_trade_use_case: MockCreateTradeUseCase) -> AppState {
    AppState {
        create_trade_use_case: Arc::new(create_trade_use_case),
        ..AppState::for_testing(Arc::new(MockStatsUseCase::new()))
    }
}

fn make_payload() -> CreateTradeRequest {
    CreateTradeRequest {
        set_code: "FDN".to_string(),
        collector_number: "87".to_string(),
        language_code: "FR".to_string(),
        foil: false,
        respondent_user_id: "user_respondent".to_string(),
        quantity: 1,
    }
}

#[tokio::test]
async fn create_trade_returns_created_on_nominal_payload() {
    let mut mock_use_case = MockCreateTradeUseCase::new();
    mock_use_case
        .expect_create_trade()
        .times(1)
        .returning(|_, _, _, _| Box::pin(async { Ok(TradeId::new()) }));

    let state = make_app_state(mock_use_case);
    let user = User::for_testing();

    let result = create_trade(
        AuthenticatedUser(user),
        State(state),
        axum::Json(make_payload()),
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StatusCode::CREATED);
}

#[tokio::test]
async fn create_trade_returns_bad_request_on_invalid_language_code() {
    let mock_use_case = MockCreateTradeUseCase::new();
    let state = make_app_state(mock_use_case);
    let mut payload = make_payload();
    payload.language_code = "XX".to_string();

    let result = create_trade(
        AuthenticatedUser(User::for_testing()),
        State(state),
        axum::Json(payload),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::WrongFormat(msg) => assert!(msg.contains("language code")),
        _ => panic!("Expected WrongFormat"),
    }
}

#[tokio::test]
async fn create_trade_returns_bad_request_on_invalid_card_id() {
    let mock_use_case = MockCreateTradeUseCase::new();
    let state = make_app_state(mock_use_case);
    let mut payload = make_payload();
    payload.collector_number = "12345678901".to_string();

    let result = create_trade(
        AuthenticatedUser(User::for_testing()),
        State(state),
        axum::Json(payload),
    )
    .await;

    assert!(matches!(result, Err(AppError::WrongFormat(_))));
}

#[tokio::test]
async fn create_trade_returns_bad_request_when_quantity_is_zero() {
    let mock_use_case = MockCreateTradeUseCase::new();
    let state = make_app_state(mock_use_case);
    let mut payload = make_payload();
    payload.quantity = 0;

    let result = create_trade(
        AuthenticatedUser(User::for_testing()),
        State(state),
        axum::Json(payload),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::WrongFormat(msg) => assert_eq!(msg, "quantity must be at least 1"),
        _ => panic!("Expected WrongFormat"),
    }
}

#[tokio::test]
async fn create_trade_propagates_card_not_found_from_use_case() {
    let mut mock_use_case = MockCreateTradeUseCase::new();
    mock_use_case
        .expect_create_trade()
        .times(1)
        .returning(|_, _, _, _| Box::pin(async { Err(AppError::CardNotFound) }));

    let state = make_app_state(mock_use_case);
    let result = create_trade(
        AuthenticatedUser(User::for_testing()),
        State(state),
        axum::Json(make_payload()),
    )
    .await;

    assert!(matches!(result, Err(AppError::CardNotFound)));
}

#[tokio::test]
async fn create_trade_propagates_self_trade_from_use_case() {
    let mut mock_use_case = MockCreateTradeUseCase::new();
    mock_use_case
        .expect_create_trade()
        .times(1)
        .returning(|_, _, _, _| Box::pin(async { Err(AppError::SelfTrade) }));

    let state = make_app_state(mock_use_case);
    let result = create_trade(
        AuthenticatedUser(User::for_testing()),
        State(state),
        axum::Json(make_payload()),
    )
    .await;

    assert!(matches!(result, Err(AppError::SelfTrade)));
}

#[tokio::test]
async fn create_trade_propagates_trade_not_modifiable_from_use_case() {
    let mut mock_use_case = MockCreateTradeUseCase::new();
    mock_use_case
        .expect_create_trade()
        .times(1)
        .returning(|_, _, _, _| Box::pin(async { Err(AppError::TradeNotModifiable) }));

    let state = make_app_state(mock_use_case);
    let result = create_trade(
        AuthenticatedUser(User::for_testing()),
        State(state),
        axum::Json(make_payload()),
    )
    .await;

    assert!(matches!(result, Err(AppError::TradeNotModifiable)));
}
