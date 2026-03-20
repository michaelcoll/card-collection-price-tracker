use crate::application::caller::EdhRecCaller;
use crate::application::service::card_collection_service::CardCollectionService;
use crate::application::service::import_card_service::ImportCardService;
use crate::application::service::import_price_service::ImportPriceService;
use crate::application::service::stats_service::StatsService;
use crate::application::service::update_card_market_service::UpdateCardMarketIdService;
use crate::application::use_case::{ImportCardUseCase, ImportPriceUseCase, StatsUseCase};
use crate::infrastructure::adapter_in::card_controller::create_card_router;
use crate::infrastructure::adapter_out::caller::cardmarket_caller_adapter::CardMarketCallerAdapter;
use crate::infrastructure::adapter_out::caller::edhrec_caller_adapter::EdhRecCallerAdapter;
use crate::infrastructure::adapter_out::repository::card_collection_repository_adapter::CardCollectionRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::cardmarket_repository_adapter::CardMarketRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::stats_repository_adapter::StatsRepositoryAdapter;
use adapter_in::stats_controller::create_stats_router;
use adapter_out::caller::scryfall_caller_adapter::ScryfallCallerAdapter;
use adapter_out::repository::card_repository_adapter::CardRepositoryAdapter;
use adapter_out::repository::set_names_repository_adapter::SetNameRepositoryAdapter;
use axum::Router;
use chrono::Utc;
use cron_tab::AsyncCron;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub mod adapter_in;
pub mod adapter_out;

#[derive(Clone)]
pub struct AppState {
    import_card_use_case: Arc<dyn ImportCardUseCase>,
    edh_rec_caller_adapter: Arc<dyn EdhRecCaller>,
    stats_use_case: Arc<dyn StatsUseCase>,
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

    let card_repository_adapter = Arc::new(CardRepositoryAdapter::new(pool.clone()));
    let set_name_repository_adapter = Arc::new(SetNameRepositoryAdapter::new(pool.clone()));
    let card_market_repository_adapter = Arc::new(CardMarketRepositoryAdapter::new(pool.clone()));
    let card_market_caller_adapter =
        Arc::new(CardMarketCallerAdapter::new(cardmarket_price_guides_url));
    let edh_rec_caller_adapter = Arc::new(EdhRecCallerAdapter::new(edh_rec_base_url));
    let scryfall_caller_adapter = Arc::new(ScryfallCallerAdapter::new(scryfall_base_url));
    let stats_repository_adapter = Arc::new(StatsRepositoryAdapter::new(pool.clone()));

    let card_collection_service = Arc::new(CardCollectionService::new(Arc::new(
        CardCollectionRepositoryAdapter::new(pool.clone()),
    )));

    let update_card_market_id_service = Arc::new(UpdateCardMarketIdService::new(
        card_repository_adapter.clone(),
        scryfall_caller_adapter,
        card_collection_service.clone(),
    ));

    let import_card_service = Arc::new(ImportCardService::new(
        card_repository_adapter.clone(),
        set_name_repository_adapter,
        update_card_market_id_service.clone(),
    ));

    let import_price_service = Arc::new(ImportPriceService::new(
        card_market_caller_adapter,
        card_market_repository_adapter,
        card_collection_service.clone(),
    ));

    let stats_service = Arc::new(StatsService::new(stats_repository_adapter));

    let app_state = AppState {
        import_card_use_case: import_card_service,
        edh_rec_caller_adapter,
        stats_use_case: stats_service,
    };

    let mut cron = AsyncCron::new(Utc);

    cron.add_fn("0 0 */6 * * *", move || {
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
        .nest("/stats", create_stats_router())
        .with_state(app_state)
}

#[cfg(test)]
impl AppState {
    pub fn for_testing(stats_use_case: Arc<dyn StatsUseCase>) -> Self {
        use crate::application::caller::MockEdhRecCaller;
        use crate::application::use_case::MockImportCardUseCase;
        use crate::domain::card::CardInfo;

        let mut mock_import_card = MockImportCardUseCase::new();
        mock_import_card
            .expect_import_cards()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_edh_rec = MockEdhRecCaller::new();
        mock_edh_rec.expect_get_card_info().returning(|_| {
            Box::pin(async {
                Ok(CardInfo {
                    inclusion: 0,
                    total_decks: 0,
                })
            })
        });

        Self {
            import_card_use_case: Arc::new(mock_import_card),
            edh_rec_caller_adapter: Arc::new(mock_edh_rec),
            stats_use_case,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::error::AppError;
    use crate::application::use_case::MockStatsUseCase;

    #[tokio::test]
    async fn for_testing_creates_app_state_with_provided_stats_use_case() {
        let mut mock_stats = MockStatsUseCase::new();
        mock_stats
            .expect_get_stats()
            .returning(|| Box::pin(async { Err(AppError::RepositoryError("".to_string())) }));

        let stats_use_case = Arc::new(mock_stats);
        let app_state = AppState::for_testing(stats_use_case);

        let result = app_state.stats_use_case.get_stats().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn for_testing_edh_rec_caller_returns_card_info_with_zero_values() {
        let mock_stats = Arc::new(MockStatsUseCase::new());
        let app_state = AppState::for_testing(mock_stats);

        let result = app_state
            .edh_rec_caller_adapter
            .get_card_info("Test Card".to_string())
            .await;

        assert!(result.is_ok());
        let card_info = result.unwrap();
        assert_eq!(card_info.inclusion, 0);
        assert_eq!(card_info.total_decks, 0);
    }

    #[tokio::test]
    async fn for_testing_import_card_use_case_succeeds_with_any_csv() {
        let mock_stats = Arc::new(MockStatsUseCase::new());
        let app_state = AppState::for_testing(mock_stats);

        let result = app_state
            .import_card_use_case
            .import_cards("any csv data")
            .await;

        assert!(result.is_ok());
    }

    #[test]
    fn for_testing_initializes_all_components() {
        let mock_stats = Arc::new(MockStatsUseCase::new());
        let app_state = AppState::for_testing(mock_stats);

        let import_card_ptr = Arc::as_ptr(&app_state.import_card_use_case);
        let edh_rec_ptr = Arc::as_ptr(&app_state.edh_rec_caller_adapter);

        assert!(!import_card_ptr.is_null());
        assert!(!edh_rec_ptr.is_null());
    }
}
