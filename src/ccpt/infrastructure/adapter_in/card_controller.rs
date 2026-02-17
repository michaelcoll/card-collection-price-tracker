use crate::infrastructure::AppState;
use axum::body::to_bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/import", post(import_cards))
        .route("/price", post(import_prices_for_current_date))
        .route("/card-info", post(get_card_info))
        .route("/update-card", post(update_card_id))
}

async fn import_cards(
    State(state): State<AppState>,
    body: axum::body::Body,
) -> Result<String, (StatusCode, String)> {
    let bytes = to_bytes(body, 10 * 1024 * 1024).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read body: {}", e),
        )
    })?;

    let csv = String::from_utf8(bytes.to_vec()).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Body is not valid UTF-8".to_string(),
        )
    })?;

    state
        .import_card_use_case
        .clone()
        .import_cards(&csv)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Import error: {:?}", e),
            )
        })?;

    Ok("Cards imported successfully".to_string())
}

async fn import_prices_for_current_date(
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    state
        .import_price_use_case
        .import_prices_for_current_date()
        .await
        .expect("panic message");

    Ok("Price imported".to_string())
}

async fn get_card_info(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
    state
        .edh_rec_caller_adapter
        .get_card_info("Sol Ring".to_string())
        .await
        .expect("panic message");

    Ok("card Info".to_string())
}

async fn update_card_id(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
    state
        .update_card_market_id_service
        .update_cards()
        .await
        .expect("panic message");

    Ok("Updated".to_string())
}
