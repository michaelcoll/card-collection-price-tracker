use crate::infrastructure::AppState;
use axum::body::to_bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;

pub fn create_card_router() -> axum::Router<AppState> {
    axum::Router::new().route("/import", post(import_cards))
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
