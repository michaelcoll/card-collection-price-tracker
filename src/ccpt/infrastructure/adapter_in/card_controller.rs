use crate::application::error::AppError;
use crate::domain::card::Card;
use crate::domain::collection::{CollectionQuery, CollectionSortField, SortDirection};
use crate::infrastructure::AppState;
use crate::infrastructure::adapter_in::auth_extractor::AuthenticatedUser;
use axum::body::to_bytes;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_collection))
        .route("/import", post(import_cards))
        .route("/card-info", post(get_card_info))
}

// --- Query params ---
#[derive(Deserialize, Default, TS)]
#[serde(rename = "SortBy", rename_all = "snake_case")]
#[ts(export, export_to = "SortBy.ts")]
enum SortByParam {
    Avg,
    #[default]
    Trend,
    SetCode,
    LanguageCode,
}

#[derive(Deserialize, Default, TS)]
#[serde(rename = "SortDir", rename_all = "snake_case")]
#[ts(export, export_to = "SortDir.ts")]
enum SortDirParam {
    Asc,
    #[default]
    Desc,
}

fn default_page_size() -> u32 {
    20
}

fn max_page_size() -> u32 {
    100
}

#[derive(Deserialize)]
struct CollectionParams {
    #[serde(default)]
    page: u32,
    #[serde(default = "default_page_size")]
    page_size: u32,
    #[serde(default)]
    sort_by: SortByParam,
    #[serde(default)]
    sort_dir: SortDirParam,
}

// --- Réponses ---
#[derive(Serialize, TS)]
#[serde(rename = "PriceGuide")]
#[ts(export, export_to = "PriceGuide.ts")]
struct PriceGuideResponse {
    low: Option<u32>,
    avg: Option<u32>,
    trend: Option<u32>,
    avg1: Option<u32>,
    avg7: Option<u32>,
    avg30: Option<u32>,
}

#[derive(Serialize, TS)]
#[serde(rename = "CollectionCard")]
#[ts(export, export_to = "CollectionCard.ts")]
struct CollectionCardResponse {
    set_code: String,
    collector_number: String,
    language_code: String,
    foil: bool,
    name: String,
    rarity_code: String,
    scryfall_id: String,
    quantity: u8,
    purchase_price: u32,
    price_guide: Option<PriceGuideResponse>,
}

#[derive(Serialize, Debug, TS)]
#[serde(rename = "Message")]
#[ts(export, export_to = "Message.ts")]
struct MessageResponse {
    message: String,
}

#[derive(Serialize, TS)]
#[serde(rename = "PaginatedCollection")]
#[ts(export, export_to = "PaginatedCollection.ts")]
struct PaginatedCollectionResponse {
    items: Vec<CollectionCardResponse>,
    total: u64,
    page: u32,
    page_size: u32,
}

impl From<Card> for CollectionCardResponse {
    fn from(c: Card) -> Self {
        Self {
            set_code: c.id.set_code.to_string(),
            collector_number: c.id.collector_number,
            language_code: c.id.language_code.to_string(),
            foil: c.id.foil,
            name: c.name,
            rarity_code: c.rarity_code.to_string(),
            scryfall_id: c.scryfall_id.to_string(),
            quantity: c.quantity,
            purchase_price: c.purchase_price,
            price_guide: c.price_guide.map(|pg| PriceGuideResponse {
                low: pg.low.value,
                avg: pg.avg.value,
                trend: pg.trend.value,
                avg1: pg.avg1.value,
                avg7: pg.avg7.value,
                avg30: pg.avg30.value,
            }),
        }
    }
}

// --- Handler ---
async fn get_collection(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Query(params): Query<CollectionParams>,
) -> Result<axum::Json<PaginatedCollectionResponse>, AppError> {
    let page_size = params.page_size.min(max_page_size());

    let query = CollectionQuery {
        page: params.page,
        page_size,
        sort_by: match params.sort_by {
            SortByParam::Avg => CollectionSortField::Avg,
            SortByParam::Trend => CollectionSortField::Trend,
            SortByParam::SetCode => CollectionSortField::SetCode,
            SortByParam::LanguageCode => CollectionSortField::LanguageCode,
        },
        sort_dir: match params.sort_dir {
            SortDirParam::Asc => SortDirection::Asc,
            SortDirParam::Desc => SortDirection::Desc,
        },
    };

    let result = state
        .get_collection_use_case
        .get_collection(&user.id, query)
        .await?;

    Ok(axum::Json(PaginatedCollectionResponse {
        items: result
            .items
            .into_iter()
            .map(CollectionCardResponse::from)
            .collect(),
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}

async fn import_cards(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    body: axum::body::Body,
) -> Result<axum::Json<MessageResponse>, AppError> {
    let bytes = to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|e| AppError::WrongFormat(format!("Failed to read body: {}", e)))?;

    let csv = String::from_utf8(bytes.to_vec())
        .map_err(|_| AppError::WrongFormat("Body is not valid UTF-8".to_string()))?;

    // L'user.id est maintenant disponible pour associer les données importées à l'utilisateur
    println!("Importing cards for user: {} ({})", user.email, user.id);

    state
        .import_card_use_case
        .clone()
        .import_cards(&csv, user)
        .await?;

    Ok(axum::Json(MessageResponse {
        message: "Cards imported successfully".to_string(),
    }))
}

async fn get_card_info(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    println!("Getting card info for user: {} ({})", user.email, user.id);

    state
        .edh_rec_caller_adapter
        .get_card_info("Sol Ring".to_string())
        .await
        .expect("panic message");

    Ok("card Info".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::error::AppError;
    use crate::application::use_case::MockGetCollectionUseCase;
    use crate::domain::card::Card;
    use crate::domain::collection::PaginatedCollection;
    use crate::domain::language_code::LanguageCode;
    use crate::domain::rarity_code::RarityCode;
    use crate::domain::user::User;
    use axum::body::Body;
    use std::sync::Arc;

    fn make_app_state_with_collection(mock: MockGetCollectionUseCase) -> AppState {
        use crate::application::caller::MockEdhRecCaller;
        use crate::application::service::auth_service::MockAuthService;
        use crate::application::use_case::{
            MockImportCardUseCase, MockImportPriceUseCase, MockStatsUseCase,
            MockUpdateCardMarketIdUseCase,
        };

        AppState {
            import_card_use_case: Arc::new(MockImportCardUseCase::new()),
            edh_rec_caller_adapter: Arc::new(MockEdhRecCaller::new()),
            stats_use_case: Arc::new(MockStatsUseCase::new()),
            auth_service: Arc::new(MockAuthService::new()),
            get_collection_use_case: Arc::new(mock),
            import_price_use_case: Arc::new(MockImportPriceUseCase::new()),
            update_card_market_id_use_case: Arc::new(MockUpdateCardMarketIdUseCase::new()),
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
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
        };

        let result = get_collection(
            AuthenticatedUser(User::for_testing()),
            State(app_state),
            Query(params),
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
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
        };

        let result = get_collection(
            AuthenticatedUser(User::for_testing()),
            State(app_state),
            Query(params),
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
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
        };

        let result = get_collection(
            AuthenticatedUser(User::for_testing()),
            State(app_state),
            Query(params),
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
            page: 0,
            page_size: 9999,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
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
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
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
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
        };

        let result = get_collection(
            AuthenticatedUser(User::for_testing()),
            State(app_state),
            Query(params),
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
        assert_eq!(item.quantity, 1);
        assert_eq!(item.purchase_price, 100);
        assert!(item.price_guide.is_none());
    }

    #[tokio::test]
    async fn get_collection_passes_sort_by_avg_to_use_case() {
        let mut mock = MockGetCollectionUseCase::new();
        mock.expect_get_collection()
            .withf(|_, q| q.sort_by == CollectionSortField::Avg)
            .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

        let app_state = make_app_state_with_collection(mock);
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::Avg,
            sort_dir: SortDirParam::default(),
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
            page: 0,
            page_size: 20,
            sort_by: SortByParam::SetCode,
            sort_dir: SortDirParam::default(),
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
            page: 0,
            page_size: 20,
            sort_by: SortByParam::LanguageCode,
            sort_dir: SortDirParam::default(),
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
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::Asc,
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
        let mut mock = MockGetCollectionUseCase::new();
        mock.expect_get_collection()
            .withf(|_, q| q.sort_dir == SortDirection::Desc)
            .returning(|_, _| Box::pin(async { Ok(make_paginated(vec![], 0, 20)) }));

        let app_state = make_app_state_with_collection(mock);
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::Desc,
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
    async fn get_collection_maps_price_guide_fields_when_card_has_prices() {
        use crate::domain::price::{Price, PriceGuide};

        let mut card = make_card("FDN", "1");
        card.price_guide = Some(PriceGuide {
            low: Price { value: Some(100) },
            avg: Price { value: Some(200) },
            trend: Price { value: Some(300) },
            avg1: Price { value: Some(400) },
            avg7: Price { value: Some(500) },
            avg30: Price { value: Some(600) },
        });

        let mut mock = MockGetCollectionUseCase::new();
        mock.expect_get_collection().returning(move |_, _| {
            let c = card.clone();
            Box::pin(async move { Ok(make_paginated(vec![c], 0, 20)) })
        });

        let app_state = make_app_state_with_collection(mock);
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
        };

        let result = get_collection(
            AuthenticatedUser(User::for_testing()),
            State(app_state),
            Query(params),
        )
        .await;

        assert!(result.is_ok());
        let axum::Json(response) = result.unwrap();
        let pg = response.items[0].price_guide.as_ref().unwrap();
        assert_eq!(pg.low, Some(100));
        assert_eq!(pg.avg, Some(200));
        assert_eq!(pg.trend, Some(300));
        assert_eq!(pg.avg1, Some(400));
        assert_eq!(pg.avg7, Some(500));
        assert_eq!(pg.avg30, Some(600));
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
        let params = CollectionParams {
            page: 0,
            page_size: 20,
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
        };

        let result = get_collection(
            AuthenticatedUser(User::for_testing()),
            State(app_state),
            Query(params),
        )
        .await;

        assert!(result.is_ok());
        let axum::Json(response) = result.unwrap();
        let item = &response.items[0];
        assert!(item.foil);
        assert_eq!(item.name, "Foil Card");
        assert_eq!(item.quantity, 2);
        assert_eq!(item.purchase_price, 500);
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
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
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
            AppError::WrongFormat(msg) => {
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
}
