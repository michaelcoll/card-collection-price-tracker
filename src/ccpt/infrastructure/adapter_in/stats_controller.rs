use crate::application::error::AppError;
use crate::domain::stats::Stats;
use crate::infrastructure::AppState;
use axum::Json;
use axum::extract::State;
use axum::routing::get;
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

pub fn create_stats_router() -> axum::Router<AppState> {
    axum::Router::new().route("/", get(get_stats))
}

async fn get_stats(State(state): State<AppState>) -> Result<Json<StatsResponse>, AppError> {
    let stats = state.stats_use_case.get_stats().await?;

    Ok(Json(stats.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::use_case::MockStatsUseCase;
    use std::sync::Arc;

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

        let app_state = AppState::for_testing(Arc::new(mock_stats_use_case));

        let result = get_stats(State(app_state)).await;

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

        let app_state = AppState::for_testing(Arc::new(mock_stats_use_case));

        let result = get_stats(State(app_state)).await;

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

        let app_state = AppState::for_testing(Arc::new(mock_stats_use_case));

        let result = get_stats(State(app_state)).await;

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

        // First call
        let result1 = get_stats(State(app_state.clone())).await;
        assert!(result1.is_ok());

        // Second call
        let result2 = get_stats(State(app_state)).await;
        assert!(result2.is_ok());
    }
}
