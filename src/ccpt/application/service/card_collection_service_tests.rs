use super::*;
use crate::application::repository::MockCollectionPriceHistoryRepository;
use crate::domain::user::User;
use chrono::NaiveDate;

#[tokio::test]
async fn calculate_total_price_succeeds_with_no_dates_and_users() {
    let mut mock_repository = MockCollectionPriceHistoryRepository::new();
    mock_repository
        .expect_get_date_and_user_to_update()
        .times(1)
        .returning(|| Box::pin(async { Ok(vec![]) }));

    let service = CardCollectionService::new(Arc::new(mock_repository));
    let result = service.calculate_total_price().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn calculate_total_price_updates_single_date_and_user() {
    let mut mock_repository = MockCollectionPriceHistoryRepository::new();

    mock_repository
        .expect_get_date_and_user_to_update()
        .times(1)
        .returning(|| {
            Box::pin(async {
                Ok(vec![(
                    NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
                    User::for_testing(),
                )])
            })
        });

    mock_repository
        .expect_update_for_date_and_user()
        .times(1)
        .returning(|_, _| Box::pin(async { Ok(()) }));

    let service = CardCollectionService::new(Arc::new(mock_repository));
    let result = service.calculate_total_price().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn calculate_total_price_updates_multiple_dates_and_users() {
    let mut mock_repository = MockCollectionPriceHistoryRepository::new();

    mock_repository
        .expect_get_date_and_user_to_update()
        .times(1)
        .returning(|| {
            Box::pin(async {
                Ok(vec![
                    (
                        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
                        User::for_testing(),
                    ),
                    (
                        NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
                        User::for_testing(),
                    ),
                    (
                        NaiveDate::from_ymd_opt(2025, 12, 27).unwrap(),
                        User::for_testing(),
                    ),
                ])
            })
        });

    mock_repository
        .expect_update_for_date_and_user()
        .times(3)
        .returning(|_, _| Box::pin(async { Ok(()) }));

    let service = CardCollectionService::new(Arc::new(mock_repository));
    let result = service.calculate_total_price().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn calculate_total_price_returns_error_when_get_dates_fails() {
    let mut mock_repository = MockCollectionPriceHistoryRepository::new();

    mock_repository
        .expect_get_date_and_user_to_update()
        .times(1)
        .returning(|| Box::pin(async { Err(AppError::CallError("database error".to_string())) }));

    let service = CardCollectionService::new(Arc::new(mock_repository));
    let result = service.calculate_total_price().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn calculate_total_price_stops_on_first_update_failure() {
    let mut mock_repository = MockCollectionPriceHistoryRepository::new();

    mock_repository
        .expect_get_date_and_user_to_update()
        .times(1)
        .returning(|| {
            Box::pin(async {
                Ok(vec![
                    (
                        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
                        User::for_testing(),
                    ),
                    (
                        NaiveDate::from_ymd_opt(2025, 12, 26).unwrap(),
                        User::for_testing(),
                    ),
                ])
            })
        });

    mock_repository
        .expect_update_for_date_and_user()
        .times(1)
        .returning(|_, _| {
            Box::pin(async { Err(AppError::CallError("update failed".to_string())) })
        });

    let service = CardCollectionService::new(Arc::new(mock_repository));
    let result = service.calculate_total_price().await;

    assert!(result.is_err());
}
