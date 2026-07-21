use super::controller::*;
use super::dto::*;
use crate::application::error::{AppError, InfraError};
use crate::domain::error::FunctionalError;
use crate::domain::user::User;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::body::Body;
use axum::extract::{Query, State};
use chrono::NaiveDate;
use std::sync::Arc;

fn make_app_state_with_stats(
    mock: crate::application::use_case::MockGetCollectionStatsUseCase,
) -> AppState {
    AppState {
        get_collection_stats_use_case: Arc::new(mock),
        ..AppState::for_testing(Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ))
    }
}

fn make_app_state_with_price_history(
    mock: crate::application::use_case::MockGetCollectionPriceHistoryUseCase,
) -> AppState {
    AppState {
        get_collection_price_history_use_case: Arc::new(mock),
        ..AppState::for_testing(Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ))
    }
}

fn make_app_state_with_card_price_history(
    mock: crate::application::use_case::MockGetCardPriceHistoryUseCase,
) -> AppState {
    AppState {
        get_card_price_history_use_case: Arc::new(mock),
        ..AppState::for_testing(Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ))
    }
}

#[tokio::test]
async fn import_cards_succeeds_with_valid_csv() {
    let app_state = AppState::for_testing(Arc::new(
        crate::application::use_case::MockStatsUseCase::new(),
    ));

    let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";

    let test_user = User::for_testing();
    let result = import_cards(
        AuthenticatedUser(test_user),
        State(app_state),
        Body::from(csv_body),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.message, "Cards imported successfully");
}

#[tokio::test]
async fn import_cards_fails_with_invalid_utf8() {
    let app_state = AppState::for_testing(Arc::new(
        crate::application::use_case::MockStatsUseCase::new(),
    ));

    // Create invalid UTF-8 bytes
    let invalid_bytes = vec![0xFF, 0xFE, 0xFD];

    let test_user = User::for_testing();
    let result = import_cards(
        AuthenticatedUser(test_user),
        State(app_state),
        Body::from(invalid_bytes),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Functional(FunctionalError::WrongFormat(msg)) => {
            assert_eq!(msg, "Body is not valid UTF-8");
        }
        _ => panic!("Expected WrongFormat error"),
    }
}

#[tokio::test]
async fn import_cards_succeeds_with_multiple_cards() {
    let app_state = AppState::for_testing(Arc::new(
        crate::application::use_case::MockStatsUseCase::new(),
    ));

    let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR\n\
        bulk,binder,Repeal,GPT,Guildpact,32,normal,common,2,27563,9e7dd929-4bba-46a6-86c9-b8ed853eb721,0.17,false,false,near_mint,fr,EUR";

    let test_user = User::for_testing();
    let result = import_cards(
        AuthenticatedUser(test_user),
        State(app_state),
        Body::from(csv_body),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.message, "Cards imported successfully");
}

#[tokio::test]
async fn import_cards_succeeds_with_foil_cards() {
    let app_state = AppState::for_testing(Arc::new(
        crate::application::use_case::MockStatsUseCase::new(),
    ));

    let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Sol Ring,FDN,Foundations,42,foil,mythic,1,101500,11111111-1111-1111-1111-111111111111,5.00,false,false,near_mint,en,EUR";

    let test_user = User::for_testing();
    let result = import_cards(
        AuthenticatedUser(test_user),
        State(app_state),
        Body::from(csv_body),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.message, "Cards imported successfully");
}

#[tokio::test]
async fn import_cards_succeeds_with_special_characters_in_card_name() {
    let app_state = AppState::for_testing(Arc::new(
        crate::application::use_case::MockStatsUseCase::new(),
    ));

    let csv_body = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,\"Dwynen, Gilt-Leaf Daen\",FDN,Foundations,217,normal,uncommon,2,100086,01c00d7b-7fac-4f8c-a1ea-de2cf4d06627,0.2,false,false,near_mint,fr,EUR";

    let test_user = User::for_testing();
    let result = import_cards(
        AuthenticatedUser(test_user),
        State(app_state),
        Body::from(csv_body),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.message, "Cards imported successfully");
}

// --- Tests for get_collection_price_history ---

#[tokio::test]
async fn get_collection_price_history_returns_entries() {
    use crate::application::use_case::MockGetCollectionPriceHistoryUseCase;
    use crate::domain::price::PriceHistoryEntry;

    let mut mock = MockGetCollectionPriceHistoryUseCase::new();
    mock.expect_get_collection_price_history()
        .returning(|_, _, _| {
            Box::pin(async {
                use crate::domain::price::{Price, PriceGuide};
                Ok(vec![PriceHistoryEntry {
                    date: NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
                    price_guide: PriceGuide {
                        low: Price { value: Some(100) },
                        trend: Price { value: Some(150) },
                        avg: Price { value: Some(130) },
                    },
                }])
            })
        });

    let app_state = make_app_state_with_price_history(mock);
    let params = PriceHistoryParams {
        start_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()),
    };

    let result = get_collection_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(entries) = result.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].low, 100);
    assert_eq!(entries[0].trend, 150);
    assert_eq!(entries[0].avg, 130);
}

#[tokio::test]
async fn get_collection_price_history_returns_empty_list() {
    use crate::application::use_case::MockGetCollectionPriceHistoryUseCase;

    let mut mock = MockGetCollectionPriceHistoryUseCase::new();
    mock.expect_get_collection_price_history()
        .returning(|_, _, _| Box::pin(async { Ok(vec![]) }));

    let app_state = make_app_state_with_price_history(mock);
    let params = PriceHistoryParams {
        start_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()),
    };

    let result = get_collection_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(entries) = result.unwrap();
    assert!(entries.is_empty());
}

#[tokio::test]
async fn get_collection_price_history_propagates_use_case_error() {
    use crate::application::use_case::MockGetCollectionPriceHistoryUseCase;

    let mut mock = MockGetCollectionPriceHistoryUseCase::new();
    mock.expect_get_collection_price_history()
        .returning(|_, _, _| {
            Box::pin(async {
                Err(AppError::Functional(FunctionalError::WrongFormat(
                    "start_date must be before or equal to end_date".to_string(),
                )))
            })
        });

    let app_state = make_app_state_with_price_history(mock);
    let params = PriceHistoryParams {
        start_date: Some(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    };

    let result = get_collection_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Functional(FunctionalError::WrongFormat(msg)) => {
            assert_eq!(msg, "start_date must be before or equal to end_date");
        }
        _ => panic!("Expected WrongFormat error"),
    }
}

#[tokio::test]
async fn get_collection_price_history_passes_missing_dates_through_to_use_case() {
    use crate::application::use_case::MockGetCollectionPriceHistoryUseCase;

    let mut mock = MockGetCollectionPriceHistoryUseCase::new();
    mock.expect_get_collection_price_history()
        .withf(|_, s, e| s.is_none() && e.is_none())
        .returning(|_, _, _| Box::pin(async { Ok(vec![]) }));

    let app_state = make_app_state_with_price_history(mock);
    let params = PriceHistoryParams {
        start_date: None,
        end_date: None,
    };

    let result = get_collection_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
}

// --- Tests for get_card_price_history ---

#[tokio::test]
async fn get_card_price_history_returns_entries() {
    use crate::application::use_case::MockGetCardPriceHistoryUseCase;
    use crate::domain::price::{Price, PriceGuide, PriceHistoryEntry};
    use uuid::Uuid;

    let scryfall_id = Uuid::new_v4();

    let mut mock = MockGetCardPriceHistoryUseCase::new();
    mock.expect_get_card_price_history().returning(|_, _, _| {
        Box::pin(async {
            Ok(vec![PriceHistoryEntry {
                date: NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
                price_guide: PriceGuide {
                    low: Price { value: Some(100) },
                    trend: Price { value: Some(150) },
                    avg: Price { value: Some(130) },
                },
            }])
        })
    });

    let app_state = make_app_state_with_card_price_history(mock);
    let params = PriceHistoryParams {
        start_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()),
    };

    let result = get_card_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        axum::extract::Path(scryfall_id),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(entries) = result.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].low, 100);
    assert_eq!(entries[0].trend, 150);
    assert_eq!(entries[0].avg, 130);
}

#[tokio::test]
async fn get_card_price_history_returns_404_when_card_not_found() {
    use crate::application::use_case::MockGetCardPriceHistoryUseCase;
    use uuid::Uuid;

    let mut mock = MockGetCardPriceHistoryUseCase::new();
    mock.expect_get_card_price_history().returning(|_, _, _| {
        Box::pin(async { Err(AppError::Functional(FunctionalError::CardNotFound)) })
    });

    let app_state = make_app_state_with_card_price_history(mock);

    let result = get_card_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        axum::extract::Path(Uuid::new_v4()),
        Query(PriceHistoryParams {
            start_date: None,
            end_date: None,
        }),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Functional(FunctionalError::CardNotFound) => {}
        other => panic!("Expected CardNotFound, got {:?}", other),
    }
}

#[tokio::test]
async fn get_card_price_history_propagates_wrong_format_error() {
    use crate::application::use_case::MockGetCardPriceHistoryUseCase;
    use uuid::Uuid;

    let mut mock = MockGetCardPriceHistoryUseCase::new();
    mock.expect_get_card_price_history().returning(|_, _, _| {
        Box::pin(async {
            Err(AppError::Functional(FunctionalError::WrongFormat(
                "start_date must be before or equal to end_date".to_string(),
            )))
        })
    });

    let app_state = make_app_state_with_card_price_history(mock);
    let params = PriceHistoryParams {
        start_date: Some(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    };

    let result = get_card_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        axum::extract::Path(Uuid::new_v4()),
        Query(params),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Functional(FunctionalError::WrongFormat(msg)) => {
            assert_eq!(msg, "start_date must be before or equal to end_date");
        }
        _ => panic!("Expected WrongFormat error"),
    }
}

#[tokio::test]
async fn get_card_price_history_returns_empty_list() {
    use crate::application::use_case::MockGetCardPriceHistoryUseCase;
    use uuid::Uuid;

    let mut mock = MockGetCardPriceHistoryUseCase::new();
    mock.expect_get_card_price_history()
        .returning(|_, _, _| Box::pin(async { Ok(vec![]) }));

    let app_state = make_app_state_with_card_price_history(mock);

    let result = get_card_price_history(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        axum::extract::Path(Uuid::new_v4()),
        Query(PriceHistoryParams {
            start_date: None,
            end_date: None,
        }),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(entries) = result.unwrap();
    assert!(entries.is_empty());
}

// --- Tests for get_collection_stats ---

#[tokio::test]
async fn get_collection_stats_returns_stats_from_use_case() {
    use crate::application::use_case::MockGetCollectionStatsUseCase;
    use crate::domain::collection_stats::CollectionStats;
    use crate::domain::price::Price;
    use crate::domain::set_name::{SetCode, SetName};

    let mut mock = MockGetCollectionStatsUseCase::new();
    mock.expect_get_collection_stats().returning(|_| {
        Box::pin(async {
            Ok(CollectionStats {
                total_cards: 42,
                unique_cards: 10,
                price_trend_min: Price::from_cents(100),
                price_trend_max: Price::from_cents(5000),
                sets: vec![SetName::new(SetCode::new("FDN"), "Foundations")],
            })
        })
    });

    let app_state = make_app_state_with_stats(mock);
    let result =
        get_collection_stats(AuthenticatedUser(User::for_testing()), State(app_state)).await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.total_cards, 42);
    assert_eq!(response.unique_cards, 10);
    assert_eq!(response.price_trend_min, Some(100));
    assert_eq!(response.price_trend_max, Some(5000));
    assert_eq!(response.sets.len(), 1);
    assert_eq!(response.sets[0].code, "FDN");
    assert_eq!(response.sets[0].name, "Foundations");
}

#[tokio::test]
async fn get_collection_stats_returns_empty_for_empty_collection() {
    use crate::application::use_case::MockGetCollectionStatsUseCase;
    use crate::domain::collection_stats::CollectionStats;
    use crate::domain::price::Price;

    let mut mock = MockGetCollectionStatsUseCase::new();
    mock.expect_get_collection_stats().returning(|_| {
        Box::pin(async {
            Ok(CollectionStats {
                total_cards: 0,
                unique_cards: 0,
                price_trend_min: Price::empty(),
                price_trend_max: Price::empty(),
                sets: vec![],
            })
        })
    });

    let app_state = make_app_state_with_stats(mock);
    let result =
        get_collection_stats(AuthenticatedUser(User::for_testing()), State(app_state)).await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.total_cards, 0);
    assert_eq!(response.unique_cards, 0);
    assert_eq!(response.price_trend_min, None);
    assert_eq!(response.price_trend_max, None);
    assert!(response.sets.is_empty());
}

#[tokio::test]
async fn get_collection_stats_propagates_error_from_use_case() {
    use crate::application::use_case::MockGetCollectionStatsUseCase;

    let mut mock = MockGetCollectionStatsUseCase::new();
    mock.expect_get_collection_stats().returning(|_| {
        Box::pin(async {
            Err(AppError::Infra(InfraError::RepositoryError(
                "db failure".to_string(),
            )))
        })
    });

    let app_state = make_app_state_with_stats(mock);
    let result =
        get_collection_stats(AuthenticatedUser(User::for_testing()), State(app_state)).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::Infra(InfraError::RepositoryError(msg)) => assert_eq!(msg, "db failure"),
        _ => panic!("Expected RepositoryError"),
    }
}
