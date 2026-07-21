use super::controller::*;
use super::dto::*;
use crate::application::error::AppError;
use crate::application::use_case::MockGetCollectionUseCase;
use crate::domain::card::{Card, CollectionEntry};
use crate::domain::collection::{CollectionSortField, PaginatedCollection, SortDirection};
use crate::domain::language_code::LanguageCode;
use crate::domain::rarity_code::RarityCode;
use crate::domain::user::User;
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::extract::{Query, State};
use std::sync::Arc;

fn make_app_state_with_collection(mock: MockGetCollectionUseCase) -> AppState {
    AppState {
        get_collection_use_case: Arc::new(mock),
        ..AppState::for_testing(Arc::new(
            crate::application::use_case::MockStatsUseCase::new(),
        ))
    }
}

fn make_card(set_code: &str, collector_number: &str) -> Card {
    Card::new(
        set_code,
        format!("Set {}", set_code),
        collector_number,
        LanguageCode::EN,
        false,
        "Test Card",
        RarityCode::C,
        1,
        100,
    )
}

fn make_paginated(items: Vec<Card>, page: u32, page_size: u32) -> PaginatedCollection {
    let total = items.len() as u64;
    PaginatedCollection {
        items,
        total,
        page,
        page_size,
    }
}

#[tokio::test]
async fn get_collection_returns_empty_response_when_collection_is_empty() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert!(response.items.is_empty());
    assert_eq!(response.total, 0);
    assert_eq!(response.page, 0);
    assert_eq!(response.page_size, 20);
}

#[tokio::test]
async fn get_collection_returns_cards_from_use_case() {
    let cards = [make_card("FDN", "1"), make_card("GPT", "32")];
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(|_, _| {
        Box::pin(async {
            Ok(make_paginated(
                vec![make_card("FDN", "1"), make_card("GPT", "32")],
                0,
                20,
            ))
        })
    });

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.items.len(), cards.len());
    assert_eq!(response.total, 2);
}

#[tokio::test]
async fn get_collection_propagates_error_from_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(|_, _| {
        Box::pin(async { Err(AppError::RepositoryError("db failure".to_string())) })
    });

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_err());
    match result.err().unwrap() {
        AppError::RepositoryError(msg) => assert_eq!(msg, "db failure"),
        _ => panic!("Expected RepositoryError"),
    }
}

#[tokio::test]
async fn get_collection_caps_page_size_at_100() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.page_size == 100)
        .returning(|_, q| {
            let page_size = q.page_size;
            Box::pin(async move { Ok(make_paginated(vec![], 0, page_size)) })
        });

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        page_size: 9999,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.page_size, 100);
}

#[tokio::test]
async fn get_collection_passes_pagination_params_to_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.page == 3 && q.page_size == 5)
        .returning(|_, q| {
            let (page, page_size) = (q.page, q.page_size);
            Box::pin(async move { Ok(make_paginated(vec![], page, page_size)) })
        });

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        page: 3,
        page_size: 5,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.page, 3);
    assert_eq!(response.page_size, 5);
}

#[tokio::test]
async fn get_collection_maps_card_fields_correctly() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(|_, _| {
        Box::pin(async { Ok(make_paginated(vec![make_card("FDN", "42")], 0, 20)) })
    });

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    let item = &response.items[0];
    assert_eq!(item.set_code, "FDN");
    assert_eq!(item.collector_number, "42");
    assert_eq!(item.language_code, "EN");
    assert!(!item.foil);
    assert_eq!(item.name, "Test Card");
    let entry = item.collection_entry.as_ref().unwrap();
    assert_eq!(entry.quantity, 1);
    assert_eq!(entry.purchase_price, 100);
    assert!(item.owner_username.is_none());
    assert!(item.price_guide.is_none());
}

#[tokio::test]
async fn get_collection_masks_collection_entry_for_cards_owned_by_another_user() {
    let mut card = make_card("FDN", "42");
    card.collection_entry = CollectionEntry::Owned {
        owner_username: "Bob".to_string(),
    };

    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(move |_, _| {
        Box::pin({
            let c = card.clone();
            async move { Ok(make_paginated(vec![c], 0, 20)) }
        })
    });

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    let item = &response.items[0];
    assert!(item.collection_entry.is_none());
    assert_eq!(item.owner_username, Some("Bob".to_string()));
}

#[tokio::test]
async fn get_collection_passes_sort_by_avg_to_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.sort_by == CollectionSortField::Avg)
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        sort_by: SortByParam::Avg,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_collection_passes_sort_by_set_code_to_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.sort_by == CollectionSortField::SetCode)
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        sort_by: SortByParam::SetCode,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_collection_passes_sort_by_language_code_to_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.sort_by == CollectionSortField::LanguageCode)
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        sort_by: SortByParam::LanguageCode,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_collection_passes_sort_dir_asc_to_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.sort_dir == SortDirection::Asc)
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        sort_dir: SortDirParam::Asc,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_collection_passes_sort_dir_desc_to_use_case() {
    // Desc is also the default sort_dir; this test only verifies the wiring
    // is honored explicitly, independent of get_collection_passes_sort_dir_asc_to_use_case.
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.sort_dir == SortDirection::Desc)
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_collection_maps_price_guide_fields_when_card_has_prices() {
    use crate::domain::price::{Price, PriceGuide};

    let mut card = make_card("FDN", "1");
    card.price_guide = Some(PriceGuide {
        low: Price { value: Some(100) },
        avg: Price { value: Some(200) },
        trend: Price { value: Some(300) },
    });

    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(move |_, _| {
        let c = card.clone();
        Box::pin(async move { Ok(make_paginated(vec![c], 0, 20)) })
    });

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    let pg = response.items[0].price_guide.as_ref().unwrap();
    assert_eq!(pg.low, Some(100));
    assert_eq!(pg.avg, Some(200));
    assert_eq!(pg.trend, Some(300));
}

#[tokio::test]
async fn get_collection_maps_foil_card_correctly() {
    use crate::domain::language_code::LanguageCode;
    use crate::domain::rarity_code::RarityCode;

    let card = Card::new(
        "FDN",
        "Set FDN",
        "99",
        LanguageCode::EN,
        true,
        "Foil Card",
        RarityCode::R,
        2,
        500,
    );

    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(move |_, _| {
        let c = card.clone();
        Box::pin(async move { Ok(make_paginated(vec![c], 0, 20)) })
    });

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    let item = &response.items[0];
    assert!(item.foil);
    assert_eq!(item.name, "Foil Card");
    let entry = item.collection_entry.as_ref().unwrap();
    assert_eq!(entry.quantity, 2);
    assert_eq!(entry.purchase_price, 500);
}

#[tokio::test]
async fn get_collection_preserves_total_independent_of_page_items() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection().returning(|_, _| {
        Box::pin(async {
            Ok(PaginatedCollection {
                items: vec![make_card("FDN", "1")],
                total: 42,
                page: 2,
                page_size: 1,
            })
        })
    });

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        page: 2,
        page_size: 1,
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.total, 42);
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.page, 2);
    assert_eq!(response.page_size, 1);
}

#[tokio::test]
async fn get_collection_passes_search_query_to_use_case() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.search_query == Some("gob".to_string()))
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);
    let params = CollectionParams {
        q: Some("gob".to_string()),
        ..Default::default()
    };

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(params),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_collection_passes_none_search_query_when_q_is_absent() {
    let mut mock = MockGetCollectionUseCase::new();
    mock.expect_get_collection()
        .withf(|_, q| q.search_query.is_none())
        .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

    let app_state = make_app_state_with_collection(mock);

    let result = get_collection(
        AuthenticatedUser(User::for_testing()),
        State(app_state),
        Query(CollectionParams::default()),
    )
    .await;

    assert!(result.is_ok());
}
