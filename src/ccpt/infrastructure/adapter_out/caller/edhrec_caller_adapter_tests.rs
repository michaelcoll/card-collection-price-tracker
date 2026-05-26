use super::*;
use crate::test_helpers::http::mock_get;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

fn create_adapter(edh_rec_base_url: impl Into<String>) -> EdhRecCallerAdapter {
    EdhRecCallerAdapter::new(edh_rec_base_url)
}

#[tokio::test]
async fn get_build_id_returns_valid_build_id() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"abc123def456","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());
    let result = adapter.get_build_id().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc123def456");
}

#[tokio::test]
async fn get_build_id_returns_error_when_script_not_found() {
    let mock_server = MockServer::start().await;

    let html_response = r#"<html><body>No script tag</body></html>"#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());
    let result = adapter.get_build_id().await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(msg) => assert!(msg.contains("unable to find")),
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn get_build_id_returns_error_when_build_id_missing() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());
    let result = adapter.get_build_id().await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(msg) => assert!(msg.contains("buildId not found")),
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn get_build_id_returns_error_on_http_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(path("/faq"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let adapter = create_adapter(mock_server.uri());
    let result = adapter.get_build_id().await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(_) => (),
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn get_build_id_returns_error_on_invalid_json() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {invalid json
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = EdhRecCallerAdapter::new(mock_server.uri());
    let result = adapter.get_build_id().await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(msg) => assert!(msg.contains("is not valid json")),
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn update_build_id_fetches_and_caches_when_cache_empty() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"fresh-build-id","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());
    let result = adapter.update_build_id().await;

    assert!(result.is_ok());
    let cache = adapter.cache.read().await;
    assert_eq!(cache.id.as_ref().unwrap(), "fresh-build-id");
    assert!(cache.last_updated.is_some());
}

#[tokio::test]
async fn update_build_id_does_not_fetch_when_cache_valid() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"new-build-id","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());

    {
        let mut cache = adapter.cache.write().await;
        cache.id = Some("cached-build-id".to_string());
        cache.last_updated = Some(Utc::now().naive_utc());
    }

    let result = adapter.update_build_id().await;

    assert!(result.is_ok());
    let cache = adapter.cache.read().await;
    assert_eq!(cache.id.as_ref().unwrap(), "cached-build-id");
}

#[tokio::test]
async fn update_build_id_refetches_when_cache_expired() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"refreshed-build-id","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());

    {
        let mut cache = adapter.cache.write().await;
        cache.id = Some("old-build-id".to_string());
        cache.last_updated = Some(Utc::now().naive_utc() - Duration::hours(25));
    }

    let result = adapter.update_build_id().await;

    assert!(result.is_ok());
    let cache = adapter.cache.read().await;
    assert_eq!(cache.id.as_ref().unwrap(), "refreshed-build-id");
}

#[tokio::test]
async fn update_build_id_returns_error_when_get_build_id_fails() {
    let mock_server = MockServer::start().await;

    Mock::given(path("/faq"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let adapter = create_adapter(mock_server.uri());
    let result = adapter.update_build_id().await;

    assert!(result.is_err());
    let cache = adapter.cache.read().await;
    assert!(cache.id.is_none());
}

#[tokio::test]
async fn update_build_id_updates_last_updated_timestamp() {
    let mock_server = MockServer::start().await;

    let html_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"test-build-id","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", html_response).await;

    let adapter = create_adapter(mock_server.uri());
    let before = Utc::now().naive_utc();

    let result = adapter.update_build_id().await;

    let after = Utc::now().naive_utc();
    assert!(result.is_ok());
    let cache = adapter.cache.read().await;
    let updated = cache.last_updated.unwrap();
    assert!(updated >= before && updated <= after);
}

#[test]
fn get_card_id_from_name_replaces_spaces_with_hyphens() {
    let adapter = create_adapter("");
    let result = adapter.get_card_id_from_name("Jace the Mind Sculptor");
    assert_eq!(result, "jace-the-mind-sculptor");
}

#[test]
fn get_card_id_from_name_replaces_colons_with_hyphens() {
    let adapter = create_adapter("");
    let result = adapter.get_card_id_from_name("Atraxa, Praetors' Voice");
    assert_eq!(result, "atraxa-praetors-voice");
}

#[test]
fn get_card_id_from_name_replaces_spaces_and_colons() {
    let adapter = create_adapter("");
    let result = adapter.get_card_id_from_name("Y'shtola, Night's Blessed");
    assert_eq!(result, "yshtola-nights-blessed");
}

#[test]
fn get_card_id_from_name_empty_string() {
    let adapter = create_adapter("");
    let result = adapter.get_card_id_from_name("");
    assert_eq!(result, "");
}

#[tokio::test]
async fn get_card_info_returns_card_info_successfully() {
    let mock_server = MockServer::start().await;

    let build_id_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"test-build-id","other":"data"}
            </script>
        </html>
        "#;

    let card_info_response = r#"
        {
            "pageProps": {
                "data": {
                    "container": {
                        "json_dict": {
                            "card": {
                                "inclusion": 1500,
                                "potential_decks": 50000
                            }
                        }
                    }
                }
            }
        }
        "#;

    mock_get(&mock_server, "/faq", build_id_response).await;

    mock_get(
        &mock_server,
        "/_next/data/test-build-id/cards/jace-the-mind-sculptor.json",
        card_info_response,
    )
    .await;

    let adapter = create_adapter(mock_server.uri());
    let card_name = "Jace the Mind Sculptor";
    let result = adapter.get_card_info(card_name.to_string()).await;

    assert!(result.is_ok());
    let card_info = result.unwrap();
    assert_eq!(card_info.inclusion, 1500);
    assert_eq!(card_info.total_decks, 50000);
}

#[tokio::test]
async fn get_card_info_returns_error_when_build_id_update_fails() {
    let mock_server = MockServer::start().await;

    Mock::given(path("/faq"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let adapter = create_adapter(mock_server.uri());
    let card_name = "Jace the Mind Sculptor";
    let result = adapter.get_card_info(card_name.to_string()).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(msg) => {
            assert_eq!(msg, "unable to find __NEXT_DATA__ script");
        }
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn get_card_info_returns_error_when_card_info_request_fails() {
    let mock_server = MockServer::start().await;

    let build_id_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"test-build-id","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", build_id_response).await;

    Mock::given(path(
        "/_next/data/test-build-id/cards/jace-the-mind-sculptor.json",
    ))
    .respond_with(ResponseTemplate::new(404))
    .mount(&mock_server)
    .await;

    let adapter = create_adapter(mock_server.uri());
    let card_name = "Jace the Mind Sculptor";
    let result = adapter.get_card_info(card_name.to_string()).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(msg) => {
            assert!(msg.contains("error decoding response body"));
        }
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn get_card_info_returns_error_when_card_info_json_invalid() {
    let mock_server = MockServer::start().await;

    let build_id_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"test-build-id","other":"data"}
            </script>
        </html>
        "#;

    mock_get(&mock_server, "/faq", build_id_response).await;

    mock_get(
        &mock_server,
        "/_next/data/test-build-id/cards/jace-the-mind-sculptor.json",
        "{invalid json",
    )
    .await;

    let adapter = create_adapter(mock_server.uri());
    let card_name = "Jace the Mind Sculptor";
    let result = adapter.get_card_info(card_name.to_string()).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::CallError(msg) => {
            assert!(msg.contains("error decoding response body"));
        }
        _ => panic!("Expected CallError"),
    }
}

#[tokio::test]
async fn get_card_info_transforms_card_name_to_card_id() {
    let mock_server = MockServer::start().await;

    let build_id_response = r#"
        <html>
            <script id="__NEXT_DATA__">
            {"buildId":"test-build-id","other":"data"}
            </script>
        </html>
        "#;

    let card_info_response = r#"
        {
            "pageProps": {
                "data": {
                    "container": {
                        "json_dict": {
                            "card": {
                                "inclusion": 100,
                                "potential_decks": 1000
                            }
                        }
                    }
                }
            }
        }
        "#;

    mock_get(&mock_server, "/faq", build_id_response).await;

    // Test avec un nom contenant des espaces et des apostrophes
    mock_get(
        &mock_server,
        "/_next/data/test-build-id/cards/test-card-name.json",
        card_info_response,
    )
    .await;

    let adapter = create_adapter(mock_server.uri());
    let card_name = "Test Card Name";
    let result = adapter.get_card_info(card_name.to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_card_info_reuses_cached_build_id() {
    let mock_server = MockServer::start().await;

    let build_id_response = r#"
            <html>
                <script id="__NEXT_DATA__">
                {"buildId":"test-build-id","other":"data"}
                </script>
            </html>
            "#;

    let card_info_response = r#"
            {
                "pageProps": {
                    "data": {
                        "container": {
                            "json_dict": {
                                "card": {
                                    "inclusion": 100,
                                    "potential_decks": 1000
                                }
                            }
                        }
                    }
                }
            }
            "#;

    Mock::given(path("/faq"))
        .respond_with(ResponseTemplate::new(200).set_body_string(build_id_response))
        .expect(1)
        .mount(&mock_server)
        .await;

    mock_get(
        &mock_server,
        "/_next/data/test-build-id/cards/jace-the-mind-sculptor.json",
        card_info_response,
    )
    .await;

    mock_get(
        &mock_server,
        "/_next/data/test-build-id/cards/lightning-bolt.json",
        card_info_response,
    )
    .await;

    let adapter = create_adapter(mock_server.uri());

    let card_name1 = "Jace the Mind Sculptor";
    let result1 = adapter.get_card_info(card_name1.to_string()).await;
    assert!(result1.is_ok());

    let card_name2 = "Lightning Bolt";
    let result2 = adapter.get_card_info(card_name2.to_string()).await;
    assert!(result2.is_ok());
}

#[tokio::test]
async fn get_card_info_with_zero_inclusion_and_decks() {
    let mock_server = MockServer::start().await;

    let build_id_response = r#"
            <html>
                <script id="__NEXT_DATA__">
                {"buildId":"test-build-id","other":"data"}
                </script>
            </html>
            "#;

    let card_info_response = r#"
            {
                "pageProps": {
                    "data": {
                        "container": {
                            "json_dict": {
                                "card": {
                                    "inclusion": 0,
                                    "potential_decks": 0
                                }
                            }
                        }
                    }
                }
            }
            "#;

    mock_get(&mock_server, "/faq", build_id_response).await;

    mock_get(
        &mock_server,
        "/_next/data/test-build-id/cards/test-card.json",
        card_info_response,
    )
    .await;

    let adapter = create_adapter(mock_server.uri());
    let card_name = "Test Card";
    let result = adapter.get_card_info(card_name.to_string()).await;

    assert!(result.is_ok());
    let card_info = result.unwrap();
    assert_eq!(card_info.inclusion, 0);
    assert_eq!(card_info.total_decks, 0);
}
