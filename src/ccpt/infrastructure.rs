use crate::application::caller::EdhRecCaller;
use crate::application::service::card_collection_service::CardCollectionService;
use crate::application::service::import_card_service::ImportCardService;
use crate::application::service::import_price_service::ImportPriceService;
use crate::application::service::update_card_market_service::UpdateCardMarketIdService;
use crate::application::use_case::{
    CardCollectionPriceCalculationUseCase, ImportCardUseCase, ImportPriceUseCase,
    UpdateCardMarketIdUseCase,
};
use crate::infrastructure::adapter_in::card_controller::create_card_router;
use crate::infrastructure::adapter_out::caller::cardmarket_caller_adapter::CardMarketCallerAdapter;
use crate::infrastructure::adapter_out::caller::edhrec_caller_adapter::EdhRecCallerAdapter;
use crate::infrastructure::adapter_out::repository::card_collection_repository_adapter::CardCollectionRepositoryAdapter;
use crate::infrastructure::adapter_out::repository::cardmarket_repository_adapter::CardMarketRepositoryAdapter;
use adapter_out::caller::scryfall_caller_adapter::ScryfallCallerAdapter;
use adapter_out::repository::card_repository_adapter::CardRepositoryAdapter;
use adapter_out::repository::set_names_repository_adapter::SetNameRepositoryAdapter;
use axum::Router;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub mod adapter_in;
pub mod adapter_out;

#[derive(Clone)]
pub struct AppState {
    import_card_use_case: Arc<dyn ImportCardUseCase>,
    import_price_use_case: Arc<dyn ImportPriceUseCase>,
    update_card_market_id_service: Arc<dyn UpdateCardMarketIdUseCase>,
    edh_rec_caller_adapter: Arc<dyn EdhRecCaller>,
    card_collection_service: Arc<dyn CardCollectionPriceCalculationUseCase>,
}

pub fn create_infra(pool: Pool<Postgres>) -> Router {
    let cardmarket_price_guides_url = std::env::var("CARDMARKET_PRICE_GUIDES_URL").unwrap_or(
        "https://downloads.s3.cardmarket.com/productCatalog/priceGuide/price_guide_1.json"
            .to_string(),
    );
    let edh_rec_base_url =
        std::env::var("EDHREC_BASE_URL").unwrap_or("https://edhrec.com".to_string());
    let scryfall_base_url =
        std::env::var("SCRYFALL_BASE_URL").unwrap_or("https://api.scryfall.com".to_string());

    let card_repository_adapter = CardRepositoryAdapter::new(pool.clone());
    let set_name_repository_adapter = SetNameRepositoryAdapter::new(pool.clone());
    let card_market_repository_adapter = CardMarketRepositoryAdapter::new(pool.clone());
    let card_market_caller_adapter = CardMarketCallerAdapter::new(cardmarket_price_guides_url);
    let edh_rec_caller_adapter = EdhRecCallerAdapter::new(edh_rec_base_url);
    let scryfall_caller_adapter = ScryfallCallerAdapter::new(scryfall_base_url);

    let import_card_service = ImportCardService::new(
        Arc::new(card_repository_adapter),
        Arc::new(set_name_repository_adapter),
    );

    let import_price_service = ImportPriceService::new(
        Arc::new(card_market_caller_adapter),
        Arc::new(card_market_repository_adapter),
    );

    let update_card_market_id_service = UpdateCardMarketIdService::new(
        Arc::new(CardRepositoryAdapter::new(pool.clone())),
        Arc::new(scryfall_caller_adapter),
    );

    let card_collection_service =
        CardCollectionService::new(Arc::new(CardCollectionRepositoryAdapter::new(pool.clone())));

    let app_state = AppState {
        import_card_use_case: Arc::new(import_card_service),
        import_price_use_case: Arc::new(import_price_service),
        update_card_market_id_service: Arc::new(update_card_market_id_service),
        edh_rec_caller_adapter: Arc::new(edh_rec_caller_adapter),
        card_collection_service: Arc::new(card_collection_service),
    };

    Router::new()
        .nest("/cards", create_card_router())
        .with_state(app_state)
}
