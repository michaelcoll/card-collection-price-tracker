use crate::application::service::import_service::ImportCardService;
use crate::application::use_case::ImportCardUseCase;
use crate::infrastructure::adapter_in::card_controller::create_card_router;
use crate::infrastructure::adapter_out::card_repository_adapter::CardRepositoryAdapter;
use crate::infrastructure::adapter_out::set_names_repository_adapter::SetNameRepositoryAdapter;
use axum::Router;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub mod adapter_in;
pub mod adapter_out;

#[derive(Clone)]
pub(crate) struct AppState {
    import_card_use_case: Arc<dyn ImportCardUseCase>,
}

pub(crate) fn create_infra(pool: Pool<Postgres>) -> Router {
    let card_repository_adapter = CardRepositoryAdapter::new(pool.clone());
    let set_name_repository_adapter = SetNameRepositoryAdapter::new(pool.clone());

    let import_card_service = ImportCardService::new(
        Arc::new(card_repository_adapter),
        Arc::new(set_name_repository_adapter),
    );

    let app_state = AppState {
        import_card_use_case: Arc::new(import_card_service),
    };

    Router::new()
        .nest("/cards", create_card_router())
        .with_state(app_state)
}
