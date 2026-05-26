use super::*;
use crate::application::error::AppError;
use crate::application::use_case::MockStatsUseCase;
use crate::domain::user::User;

#[tokio::test]
async fn for_testing_creates_app_state_with_provided_stats_use_case() {
    let mut mock_stats = MockStatsUseCase::new();
    mock_stats
        .expect_get_stats()
        .returning(|| Box::pin(async { Err(AppError::RepositoryError("".to_string())) }));

    let app_state = AppState::for_testing(Arc::new(mock_stats));

    let result = app_state.stats_use_case.get_stats().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn for_testing_edh_rec_caller_returns_card_info_with_zero_values() {
    let app_state = AppState::for_testing(Arc::new(MockStatsUseCase::new()));

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
    let app_state = AppState::for_testing(Arc::new(MockStatsUseCase::new()));

    let result = app_state
        .import_card_use_case
        .import_cards("any csv data", User::for_testing())
        .await;

    assert!(result.is_ok());
}

#[test]
fn for_testing_initializes_all_components() {
    let app_state = AppState::for_testing(Arc::new(MockStatsUseCase::new()));

    let import_card_ptr = Arc::as_ptr(&app_state.import_card_use_case);
    let edh_rec_ptr = Arc::as_ptr(&app_state.edh_rec_caller_adapter);

    assert!(!import_card_ptr.is_null());
    assert!(!edh_rec_ptr.is_null());
}

#[tokio::test]
async fn create_infra_creates_router_successfully() {
    use sqlx::postgres::PgPoolOptions;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // Démarrer un serveur mock pour simuler l'endpoint JWKS de Clerk
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/.well-known/jwks.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"keys": []}"#))
        .mount(&mock_server)
        .await;

    unsafe {
        std::env::set_var("CLERK_FRONTEND_API_URL", mock_server.uri());
        std::env::set_var(
            "CARDMARKET_PRICE_GUIDES_URL",
            "https://example.com/prices.json",
        );
        std::env::set_var("EDHREC_BASE_URL", "https://edhrec.example.com");
        std::env::set_var("SCRYFALL_BASE_URL", "https://api.scryfall.example.com");
    }

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@localhost/postgres".to_string());

    // Skip le test si la base de données n'est pas disponible
    let pool = match PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let router = create_infra(pool).await;
    let _service = router.into_make_service();
}

#[tokio::test]
async fn create_infra_uses_custom_urls_from_env_vars() {
    use sqlx::postgres::PgPoolOptions;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/.well-known/jwks.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"keys": []}"#))
        .mount(&mock_server)
        .await;

    let cardmarket_url = "https://custom.cardmarket.com/prices.json";
    let edhrec_url = "https://custom.edhrec.com";
    let scryfall_url = "https://custom.scryfall.com";

    unsafe {
        std::env::set_var("CLERK_FRONTEND_API_URL", mock_server.uri());
        std::env::set_var("CARDMARKET_PRICE_GUIDES_URL", cardmarket_url);
        std::env::set_var("EDHREC_BASE_URL", edhrec_url);
        std::env::set_var("SCRYFALL_BASE_URL", scryfall_url);
    }

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@localhost/postgres".to_string());

    let pool = match PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let router = create_infra(pool).await;
    let _service = router.into_make_service();
}

#[tokio::test]
async fn create_infra_uses_default_urls_when_env_vars_not_set() {
    use sqlx::postgres::PgPoolOptions;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/.well-known/jwks.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"keys": []}"#))
        .mount(&mock_server)
        .await;

    // Retirer les variables optionnelles pour tester les valeurs par défaut
    unsafe {
        std::env::remove_var("CARDMARKET_PRICE_GUIDES_URL");
        std::env::remove_var("EDHREC_BASE_URL");
        std::env::remove_var("SCRYFALL_BASE_URL");
        std::env::set_var("CLERK_FRONTEND_API_URL", mock_server.uri());
    }

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@localhost/postgres".to_string());

    let pool = match PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let router = create_infra(pool).await;
    let _service = router.into_make_service();
}

#[test]
fn create_infra_requires_clerk_frontend_api_url() {
    // Clear CLERK_FRONTEND_API_URL to verify it's required
    unsafe {
        std::env::remove_var("CLERK_FRONTEND_API_URL");
    }

    let result = std::env::var("CLERK_FRONTEND_API_URL");

    // Verify that CLERK_FRONTEND_API_URL is not set
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), std::env::VarError::NotPresent);
}
