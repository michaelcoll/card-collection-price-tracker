use crate::application::error::AppError;
use crate::domain::stats::Stats;
use crate::infrastructure::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct StatsResponse {
    pub card_number: u32,
    pub card_price_number: u32,
    pub db_size_mb: u16,
}

impl From<Stats> for StatsResponse {
    fn from(stats: Stats) -> Self {
        Self {
            card_number: stats.card_number,
            card_price_number: stats.card_price_number,
            db_size_mb: stats.db_size_mb,
        }
    }
}

pub fn create_maintenance_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/stats", get(get_stats))
        .route("/trigger-price-update", post(trigger_price_update))
        .route("/update-cardmarket-ids", post(update_cardmarket_ids))
}

async fn get_stats(State(state): State<AppState>) -> Result<Json<StatsResponse>, AppError> {
    let stats = state.stats_use_case.get_stats().await?;
    Ok(Json(stats.into()))
}

async fn trigger_price_update(State(state): State<AppState>) -> Result<StatusCode, AppError> {
    state
        .import_price_use_case
        .import_prices_for_current_date()
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn update_cardmarket_ids(State(state): State<AppState>) -> Result<StatusCode, AppError> {
    state.update_card_market_id_use_case.update_cards().await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::use_case::{MockImportPriceUseCase, MockStatsUseCase};
    use std::sync::Arc;

    // --- Stats ---

    #[test]
    fn test_stats_response_from_stats_with_zero_values() {
        let stats = Stats {
            card_number: 0,
            card_price_number: 0,
            db_size_mb: 0,
        };
        let response: StatsResponse = stats.into();
        assert_eq!(response.card_number, 0);
        assert_eq!(response.card_price_number, 0);
        assert_eq!(response.db_size_mb, 0);
    }

    #[test]
    fn test_stats_response_from_stats_with_values() {
        let stats = Stats {
            card_number: 100,
            card_price_number: 150,
            db_size_mb: 500,
        };
        let response: StatsResponse = stats.into();
        assert_eq!(response.card_number, 100);
        assert_eq!(response.card_price_number, 150);
        assert_eq!(response.db_size_mb, 500);
    }

    #[test]
    fn test_stats_response_from_stats_with_max_values() {
        let stats = Stats {
            card_number: u32::MAX,
            card_price_number: u32::MAX,
            db_size_mb: u16::MAX,
        };
        let response: StatsResponse = stats.into();
        assert_eq!(response.card_number, u32::MAX);
        assert_eq!(response.card_price_number, u32::MAX);
        assert_eq!(response.db_size_mb, u16::MAX);
    }

    #[tokio::test]
    async fn test_get_stats_returns_stats_response_on_success() {
        let mut mock_stats_use_case = MockStatsUseCase::new();
        mock_stats_use_case
            .expect_get_stats()
            .times(1)
            .returning(|| {
                Box::pin(async {
                    Ok(Stats {
                        card_number: 42,
                        card_price_number: 85,
                        db_size_mb: 128,
                    })
                })
            });

        let result = get_stats(State(AppState::for_testing(Arc::new(mock_stats_use_case)))).await;

        assert!(result.is_ok());
        let Json(response) = result.unwrap();
        assert_eq!(response.card_number, 42);
        assert_eq!(response.card_price_number, 85);
        assert_eq!(response.db_size_mb, 128);
    }

    #[tokio::test]
    async fn test_get_stats_returns_error_on_repository_error() {
        let mut mock_stats_use_case = MockStatsUseCase::new();
        mock_stats_use_case
            .expect_get_stats()
            .times(1)
            .returning(|| {
                Box::pin(async { Err(AppError::RepositoryError("DB error".to_string())) })
            });

        let result = get_stats(State(AppState::for_testing(Arc::new(mock_stats_use_case)))).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::RepositoryError(msg) => assert_eq!(msg, "DB error"),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_get_stats_returns_error_on_price_not_found() {
        let mut mock_stats_use_case = MockStatsUseCase::new();
        mock_stats_use_case
            .expect_get_stats()
            .times(1)
            .returning(|| Box::pin(async { Err(AppError::PriceNotFound) }));

        let result = get_stats(State(AppState::for_testing(Arc::new(mock_stats_use_case)))).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::PriceNotFound => {}
            _ => panic!("Expected PriceNotFound"),
        }
    }

    #[tokio::test]
    async fn test_get_stats_with_multiple_calls() {
        let mut mock_stats_use_case = MockStatsUseCase::new();
        mock_stats_use_case
            .expect_get_stats()
            .times(2)
            .returning(|| {
                Box::pin(async {
                    Ok(Stats {
                        card_number: 10,
                        card_price_number: 20,
                        db_size_mb: 30,
                    })
                })
            });

        let app_state = AppState::for_testing(Arc::new(mock_stats_use_case));
        assert!(get_stats(State(app_state.clone())).await.is_ok());
        assert!(get_stats(State(app_state)).await.is_ok());
    }

    // --- Trigger price update ---

    #[tokio::test]
    async fn test_trigger_price_update_returns_no_content_on_success() {
        let mut mock_import_price = MockImportPriceUseCase::new();
        mock_import_price
            .expect_import_prices_for_current_date()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        let app_state = AppState::for_testing_with_import_price(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_import_price),
        );

        let result = trigger_price_update(State(app_state)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn trigger_price_update_returns_price_not_found_error() {
        let mut mock_import_price = MockImportPriceUseCase::new();
        mock_import_price
            .expect_import_prices_for_current_date()
            .times(1)
            .returning(|| Box::pin(async { Err(AppError::PriceNotFound) }));

        let app_state = AppState::for_testing_with_import_price(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_import_price),
        );

        let result = trigger_price_update(State(app_state)).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::PriceNotFound => {}
            _ => panic!("Expected PriceNotFound"),
        }
    }

    #[tokio::test]
    async fn trigger_price_update_can_be_called_multiple_times_successfully() {
        let mut mock_import_price = MockImportPriceUseCase::new();
        mock_import_price
            .expect_import_prices_for_current_date()
            .times(2)
            .returning(|| Box::pin(async { Ok(()) }));

        let app_state = AppState::for_testing_with_import_price(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_import_price),
        );

        assert_eq!(
            trigger_price_update(State(app_state.clone()))
                .await
                .unwrap(),
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            trigger_price_update(State(app_state)).await.unwrap(),
            StatusCode::NO_CONTENT
        );
    }

    #[test]
    fn stats_response_serializes_to_json_with_snake_case_field_names() {
        let response = StatsResponse {
            card_number: 12,
            card_price_number: 34,
            db_size_mb: 5,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"card_number\""));
        assert!(json.contains("\"card_price_number\""));
        assert!(json.contains("\"db_size_mb\""));
        assert!(json.contains("12"));
        assert!(json.contains("34"));
        assert!(json.contains("5"));
    }

    #[test]
    fn stats_response_from_stats_preserves_all_field_values_independently() {
        let stats = Stats {
            card_number: 11,
            card_price_number: 22,
            db_size_mb: 33,
        };
        let response: StatsResponse = stats.into();
        assert_eq!(response.card_number, 11);
        assert_eq!(response.card_price_number, 22);
        assert_eq!(response.db_size_mb, 33);
    }

    #[tokio::test]
    async fn get_stats_returns_error_when_use_case_returns_call_error() {
        let mut mock_stats_use_case = MockStatsUseCase::new();
        mock_stats_use_case
            .expect_get_stats()
            .times(1)
            .returning(|| {
                Box::pin(async { Err(AppError::CallError("external service down".to_string())) })
            });

        let result = get_stats(State(AppState::for_testing(Arc::new(mock_stats_use_case)))).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::CallError(msg) => assert_eq!(msg, "external service down"),
            _ => panic!("Expected CallError"),
        }
    }

    #[test]
    fn stats_response_debug_format_contains_field_values() {
        let response = StatsResponse {
            card_number: 5,
            card_price_number: 10,
            db_size_mb: 3,
        };
        let debug = format!("{:?}", response);
        assert!(debug.contains("5"));
        assert!(debug.contains("10"));
        assert!(debug.contains("3"));
    }

    #[tokio::test]
    async fn test_trigger_price_update_returns_error_on_failure() {
        let mut mock_import_price = MockImportPriceUseCase::new();
        mock_import_price
            .expect_import_prices_for_current_date()
            .times(1)
            .returning(|| {
                Box::pin(async { Err(AppError::RepositoryError("Import failed".to_string())) })
            });

        let app_state = AppState::for_testing_with_import_price(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_import_price),
        );

        let result = trigger_price_update(State(app_state)).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::RepositoryError(msg) => assert_eq!(msg, "Import failed"),
            _ => panic!("Expected RepositoryError"),
        }
    }

    // --- Update CardMarket IDs ---

    #[tokio::test]
    async fn test_update_cardmarket_ids_returns_no_content_on_success() {
        use crate::application::use_case::MockUpdateCardMarketIdUseCase;

        let mut mock_update = MockUpdateCardMarketIdUseCase::new();
        mock_update
            .expect_update_cards()
            .times(1)
            .returning(|| Box::pin(async { Ok(()) }));

        let app_state = AppState::for_testing_with_update_cardmarket_id(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_update),
        );

        let result = update_cardmarket_ids(State(app_state)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_update_cardmarket_ids_returns_error_on_repository_error() {
        use crate::application::use_case::MockUpdateCardMarketIdUseCase;

        let mut mock_update = MockUpdateCardMarketIdUseCase::new();
        mock_update.expect_update_cards().times(1).returning(|| {
            Box::pin(async { Err(AppError::RepositoryError("DB error".to_string())) })
        });

        let app_state = AppState::for_testing_with_update_cardmarket_id(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_update),
        );

        let result = update_cardmarket_ids(State(app_state)).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::RepositoryError(msg) => assert_eq!(msg, "DB error"),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_update_cardmarket_ids_can_be_called_multiple_times_successfully() {
        use crate::application::use_case::MockUpdateCardMarketIdUseCase;

        let mut mock_update = MockUpdateCardMarketIdUseCase::new();
        mock_update
            .expect_update_cards()
            .times(2)
            .returning(|| Box::pin(async { Ok(()) }));

        let app_state = AppState::for_testing_with_update_cardmarket_id(
            Arc::new(MockStatsUseCase::new()),
            Arc::new(mock_update),
        );

        assert_eq!(
            update_cardmarket_ids(State(app_state.clone()))
                .await
                .unwrap(),
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            update_cardmarket_ids(State(app_state)).await.unwrap(),
            StatusCode::NO_CONTENT
        );
    }
}
