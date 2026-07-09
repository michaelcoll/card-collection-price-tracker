use crate::application::error::AppError;
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
        return Err(AppError::WrongFormat(
            "Missing username claim in token".to_string(),
        ));
    }

    state.register_user_use_case.register_user(&user).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::caller::MockEdhRecCaller;
    use crate::application::service::auth_service::MockAuthService;
    use crate::application::use_case::{
        MockEnqueueCardMarketIdUpdateUseCase, MockEnqueueGathererIdUpdateUseCase,
        MockGetCollectionPriceHistoryUseCase, MockGetCollectionStatsUseCase,
        MockGetCollectionUseCase, MockImportCardUseCase, MockImportPriceUseCase,
        MockRegisterUserUseCase, MockStatsUseCase,
    };
    use crate::domain::user::User;
    use std::sync::Arc;

    fn make_app_state(register_user_use_case: MockRegisterUserUseCase) -> AppState {
        AppState {
            import_card_use_case: Arc::new(MockImportCardUseCase::new()),
            edh_rec_caller_adapter: Arc::new(MockEdhRecCaller::new()),
            stats_use_case: Arc::new(MockStatsUseCase::new()),
            auth_service: Arc::new(MockAuthService::new()),
            get_collection_use_case: Arc::new(MockGetCollectionUseCase::new()),
            import_price_use_case: Arc::new(MockImportPriceUseCase::new()),
            enqueue_cardmarket_id_use_case: Arc::new(MockEnqueueCardMarketIdUpdateUseCase::new()),
            enqueue_gatherer_id_use_case: Arc::new(MockEnqueueGathererIdUpdateUseCase::new()),
            get_collection_price_history_use_case: Arc::new(
                MockGetCollectionPriceHistoryUseCase::new(),
            ),
            get_collection_stats_use_case: Arc::new(MockGetCollectionStatsUseCase::new()),
            register_user_use_case: Arc::new(register_user_use_case),
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
            .returning(|_| {
                Box::pin(async { Err(AppError::RepositoryError("DB error".to_string())) })
            });

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
}
