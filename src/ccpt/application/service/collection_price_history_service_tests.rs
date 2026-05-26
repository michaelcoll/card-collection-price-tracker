use super::*;
use crate::application::repository::MockCollectionPriceHistoryRepository;
use crate::domain::price::{Price, PriceGuide};
use chrono::NaiveDate;

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

#[tokio::test]
async fn returns_history_from_repository() {
    let mut mock = MockCollectionPriceHistoryRepository::new();
    mock.expect_get_price_history()
        .withf(|uid, s, e| {
            uid == "user1"
                && *s == NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                && *e == NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()
        })
        .returning(|_, _, _| {
            Box::pin(async {
                Ok(vec![PriceHistoryEntry {
                    date: NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
                    price_guide: PriceGuide {
                        low: Price { value: Some(100) },
                        trend: Price { value: Some(150) },
                        avg: Price { value: Some(130) },
                        avg1: Price { value: Some(120) },
                        avg7: Price { value: Some(140) },
                        avg30: Price { value: Some(135) },
                    },
                }])
            })
        });

    let service = CollectionPriceHistoryService::new(Arc::new(mock));
    let result = service
        .get_collection_price_history("user1", date(2025, 1, 1), date(2025, 1, 31))
        .await;

    assert!(result.is_ok());
    let entries = result.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].price_guide.low.value, Some(100));
    assert_eq!(entries[0].price_guide.trend.value, Some(150));
    assert_eq!(entries[0].price_guide.avg.value, Some(130));
}

#[tokio::test]
async fn returns_error_when_start_after_end() {
    let mock = MockCollectionPriceHistoryRepository::new();
    let service = CollectionPriceHistoryService::new(Arc::new(mock));

    let result = service
        .get_collection_price_history("user1", date(2025, 2, 1), date(2025, 1, 1))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::WrongFormat(msg) => {
            assert_eq!(msg, "start_date must be before or equal to end_date");
        }
        _ => panic!("Expected WrongFormat error"),
    }
}

#[tokio::test]
async fn accepts_same_start_and_end_date() {
    let mut mock = MockCollectionPriceHistoryRepository::new();
    mock.expect_get_price_history()
        .returning(|_, _, _| Box::pin(async { Ok(vec![]) }));

    let service = CollectionPriceHistoryService::new(Arc::new(mock));
    let result = service
        .get_collection_price_history("user1", date(2025, 6, 1), date(2025, 6, 1))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn propagates_repository_error() {
    let mut mock = MockCollectionPriceHistoryRepository::new();
    mock.expect_get_price_history().returning(|_, _, _| {
        Box::pin(async { Err(AppError::RepositoryError("db error".to_string())) })
    });

    let service = CollectionPriceHistoryService::new(Arc::new(mock));
    let result = service
        .get_collection_price_history("user1", date(2025, 1, 1), date(2025, 1, 31))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::RepositoryError(msg) => assert_eq!(msg, "db error"),
        _ => panic!("Expected RepositoryError"),
    }
}
