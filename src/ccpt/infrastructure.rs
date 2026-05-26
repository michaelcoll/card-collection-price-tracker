use crate::application::caller::EdhRecCaller;
use crate::application::service::auth_service::AuthService;
use crate::application::service::card_collection_service::CardCollectionService;
use crate::application::service::cardmarket_id_enqueue_service::CardMarketIdEnqueueService;
use crate::application::service::collection_price_history_service::CollectionPriceHistoryService;
use crate::application::service::collection_service::CollectionService;
use crate::application::service::import_card_service::ImportCardService;
use crate::application::service::import_price_service::ImportPriceService;
use crate::application::service::stats_service::StatsService;
use crate::application::service::update_card_market_service::CardMarketIdWorker;
use crate::application::use_case::{
    EnqueueCardMarketIdUpdateUseCase, GetCollectionPriceHistoryUseCase, GetCollectionUseCase,
    ImportCardUseCase, ImportPriceUseCase, StatsUseCase,
};
use crate::domain::card::CardId;
use crate::infrastructure::adapter_in::card_controller::create_card_router;
use crate::infrastructure::adapter_out::caller::cardmarket_caller_adapter::CardMarketCallerAdapter;
use crate::infrastructure::adapter_out::caller::edhrec_caller_adapter::EdhRecCallerAdapter;
use crate::infrastructure::adapter_out::repository::card_prices_view_repository_adapter::CardPricesViewRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::cardmarket_price_repository_adapter::CardMarketPriceRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::collection_price_history_repository_adapter::CollectionPriceHistoryRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::collection_repository_adapter::CollectionRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::stats_repository_adapter::StatsRepositoryAdapter;
use adapter_in::maintenance_controller::create_maintenance_router;
use adapter_out::caller::scryfall_caller_adapter::ScryfallCallerAdapter;
use adapter_out::repository::card_repository_adapter::CardRepositoryAdapter;
use adapter_out::repository::set_names_repository_adapter::SetNameRepositoryAdapter;
use axum::Router;
use chrono::Utc;
use cron_tab::AsyncCron;
use sqlx::{Pool, Postgres};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub mod adapter_in;
pub mod adapter_out;

// ---- AppState ----
#[derive(Clone)]
pub struct AppState {
    pub import_card_use_case: Arc<dyn ImportCardUseCase>,
    pub edh_rec_caller_adapter: Arc<dyn EdhRecCaller>,
    pub stats_use_case: Arc<dyn StatsUseCase>,
    pub auth_service: Arc<dyn AuthService>,
    pub get_collection_use_case: Arc<dyn GetCollectionUseCase>,
    pub import_price_use_case: Arc<dyn ImportPriceUseCase>,
    pub enqueue_cardmarket_id_use_case: Arc<dyn EnqueueCardMarketIdUpdateUseCase>,
    pub get_collection_price_history_use_case: Arc<dyn GetCollectionPriceHistoryUseCase>,
}

pub async fn create_infra(pool: Pool<Postgres>) -> Router {
    let cardmarket_price_guides_url = std::env::var("CARDMARKET_PRICE_GUIDES_URL").unwrap_or(
        "https://downloads.s3.cardmarket.com/productCatalog/priceGuide/price_guide_1.json"
            .to_string(),
    );
    let edh_rec_base_url =
        std::env::var("EDHREC_BASE_URL").unwrap_or("https://edhrec.com".to_string());
    let scryfall_base_url =
        std::env::var("SCRYFALL_BASE_URL").unwrap_or("https://api.scryfall.com".to_string());
    let clerk_frontend_api_url = std::env::var("CLERK_FRONTEND_API_URL")
        .expect("CLERK_FRONTEND_API_URL must be set in environment variables");

    let card_repository_adapter = Arc::new(CardRepositoryAdapter::new(pool.clone()));
    let set_name_repository_adapter = Arc::new(SetNameRepositoryAdapter::new(pool.clone()));
    let card_market_repository_adapter =
        Arc::new(CardMarketPriceRepositoryAdapter::new(pool.clone()));
    let card_prices_view_repository_adapter =
        Arc::new(CardPricesViewRepositoryAdapter::new(pool.clone()));
    let card_market_caller_adapter =
        Arc::new(CardMarketCallerAdapter::new(cardmarket_price_guides_url));
    let edh_rec_caller_adapter = Arc::new(EdhRecCallerAdapter::new(edh_rec_base_url));
    let scryfall_caller_adapter = Arc::new(ScryfallCallerAdapter::new(scryfall_base_url));
    let stats_repository_adapter = Arc::new(StatsRepositoryAdapter::new(pool.clone()));
    let collection_repository_adapter = Arc::new(CollectionRepositoryAdapter::new(pool.clone()));

    let auth_service: Arc<dyn AuthService> = Arc::new(
        crate::application::service::auth_service::ClerkAuthService::new(
            clerk_frontend_api_url,
            None,
        )
        .await
        .expect("Failed to initialize Clerk Auth Service"),
    );

    let card_collection_service = Arc::new(CardCollectionService::new(Arc::new(
        CollectionPriceHistoryRepositoryAdapter::new(pool.clone()),
    )));

    // Canal non borné + HashSet de déduplication partagé entre enqueue service et worker
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<(CardId, Uuid)>();
    let dedup_set = Arc::new(Mutex::new(HashSet::<CardId>::new()));

    let enqueue_cardmarket_id_service = Arc::new(CardMarketIdEnqueueService::new(
        card_repository_adapter.clone(),
        sender,
        dedup_set.clone(),
    ));

    let worker = CardMarketIdWorker::new(
        card_repository_adapter.clone(),
        scryfall_caller_adapter,
        card_collection_service.clone(),
        card_prices_view_repository_adapter.clone(),
        dedup_set,
    );
    tokio::spawn(worker.run(receiver));

    let import_card_service = Arc::new(ImportCardService::new(
        card_repository_adapter.clone(),
        set_name_repository_adapter,
        enqueue_cardmarket_id_service.clone(),
        card_prices_view_repository_adapter.clone(),
    ));

    let import_price_service: Arc<dyn ImportPriceUseCase> = Arc::new(ImportPriceService::new(
        card_market_caller_adapter,
        card_market_repository_adapter,
        card_prices_view_repository_adapter,
        card_collection_service.clone(),
    ));

    let stats_service = Arc::new(StatsService::new(stats_repository_adapter));
    let collection_service = Arc::new(CollectionService::new(collection_repository_adapter));
    let collection_price_history_service: Arc<dyn GetCollectionPriceHistoryUseCase> =
        Arc::new(CollectionPriceHistoryService::new(Arc::new(
            CollectionPriceHistoryRepositoryAdapter::new(pool.clone()),
        )));

    let app_state = AppState {
        import_card_use_case: import_card_service,
        edh_rec_caller_adapter,
        stats_use_case: stats_service,
        auth_service,
        get_collection_use_case: collection_service,
        import_price_use_case: import_price_service.clone(),
        enqueue_cardmarket_id_use_case: enqueue_cardmarket_id_service,
        get_collection_price_history_use_case: collection_price_history_service,
    };

    let mut cron = AsyncCron::new(Utc);

    cron.add_fn("0 0 */12 * * *", move || {
        let service = import_price_service.clone();
        async move {
            service
                .import_prices_for_current_date()
                .await
                .expect("Failed to import prices");
        }
    })
    .await
    .unwrap();

    cron.start().await;

    Router::new()
        .nest("/cards", create_card_router())
        .nest("/maintenance", create_maintenance_router())
        .with_state(app_state)
}

#[cfg(test)]
impl AppState {
    pub fn for_testing(stats_use_case: Arc<dyn StatsUseCase>) -> Self {
        use crate::application::use_case::MockImportPriceUseCase;
        let mut mock_import_price = MockImportPriceUseCase::new();
        mock_import_price
            .expect_import_prices_for_current_date()
            .returning(|| Box::pin(async { Ok(()) }));
        Self::for_testing_with_import_price(stats_use_case, Arc::new(mock_import_price))
    }

    pub fn for_testing_with_import_price(
        stats_use_case: Arc<dyn StatsUseCase>,
        import_price_use_case: Arc<dyn ImportPriceUseCase>,
    ) -> Self {
        use crate::application::caller::MockEdhRecCaller;
        use crate::application::service::auth_service::MockAuthService;
        use crate::application::use_case::{
            MockEnqueueCardMarketIdUpdateUseCase, MockGetCollectionPriceHistoryUseCase,
            MockGetCollectionUseCase, MockImportCardUseCase,
        };
        use crate::domain::card::CardInfo;
        use crate::domain::user::User;

        let mut mock_import_card = MockImportCardUseCase::new();
        mock_import_card
            .expect_import_cards()
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let mut mock_edh_rec = MockEdhRecCaller::new();
        mock_edh_rec.expect_get_card_info().returning(|_| {
            Box::pin(async {
                Ok(CardInfo {
                    inclusion: 0,
                    total_decks: 0,
                })
            })
        });

        let mut mock_auth = MockAuthService::new();
        mock_auth.expect_validate_token().returning(|_| {
            Ok(User::new(
                "test-user-id".to_string(),
                "test@example.com".to_string(),
                None,
            ))
        });

        Self {
            import_card_use_case: Arc::new(mock_import_card),
            edh_rec_caller_adapter: Arc::new(mock_edh_rec),
            stats_use_case,
            auth_service: Arc::new(mock_auth),
            get_collection_use_case: Arc::new(MockGetCollectionUseCase::new()),
            import_price_use_case,
            enqueue_cardmarket_id_use_case: Arc::new(MockEnqueueCardMarketIdUpdateUseCase::new()),
            get_collection_price_history_use_case: Arc::new(
                MockGetCollectionPriceHistoryUseCase::new(),
            ),
        }
    }

    pub fn for_testing_with_enqueue_cardmarket_id(
        stats_use_case: Arc<dyn StatsUseCase>,
        enqueue_cardmarket_id_use_case: Arc<dyn EnqueueCardMarketIdUpdateUseCase>,
    ) -> Self {
        use crate::application::use_case::MockImportPriceUseCase;
        let mut mock_import_price = MockImportPriceUseCase::new();
        mock_import_price
            .expect_import_prices_for_current_date()
            .returning(|| Box::pin(async { Ok(()) }));
        let mut base =
            Self::for_testing_with_import_price(stats_use_case, Arc::new(mock_import_price));
        base.enqueue_cardmarket_id_use_case = enqueue_cardmarket_id_use_case;
        base
    }
}

#[cfg(test)]
#[path = "infrastructure_tests.rs"]
mod tests;
