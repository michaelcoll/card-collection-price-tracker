use super::*;
use uuid::Uuid;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_card_market_id_returns_cardmarket_id() {
    let mock_server = MockServer::start().await;

    let card_id = Uuid::default();
    let cardmarket_id = 12345;
    let response_body = format!(r#"{{ "cardmarket_id": {} }}"#, cardmarket_id);

    Mock::given(path(format!("/cards/{}", card_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let adapter = ScryfallCallerAdapter::new(mock_server.uri());
    let result = adapter.get_card_market_id(card_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(cardmarket_id));
}

#[tokio::test]
async fn get_card_market_id_returns_none_for_missing_cardmarket_id() {
    let mock_server = MockServer::start().await;

    let card_id = Uuid::default();
    let response_body = r#"{ "cardmarket_id": null }"#;

    Mock::given(path(format!("/cards/{}", card_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let adapter = ScryfallCallerAdapter::new(mock_server.uri());
    let result = adapter.get_card_market_id(card_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn get_card_market_id_returns_none_for_invalid_response() {
    let mock_server = MockServer::start().await;

    let card_id = Uuid::default();
    let response_body = r#"{ "invalid_field": "invalid_value" }"#;

    Mock::given(path(format!("/cards/{}", card_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let adapter = ScryfallCallerAdapter::new(mock_server.uri());
    let result = adapter.get_card_market_id(card_id).await;

    assert!(result.is_ok());
    if let Ok(id) = result {
        assert!(id.is_none());
    }
}
