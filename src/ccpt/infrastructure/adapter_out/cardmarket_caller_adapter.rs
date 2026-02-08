use crate::application::caller::CardMarketCaller;
use crate::application::error::AppError;
use crate::domain::cardmarket::PriceGuides;
use async_trait::async_trait;

pub struct CardMarketCallerAdapter {
    pub client: reqwest::Client,
    pub url: String,
}

impl CardMarketCallerAdapter {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: url.into(),
        }
    }
}

#[async_trait]
impl CardMarketCaller for CardMarketCallerAdapter {
    async fn get_price_guides(&self) -> Result<PriceGuides, AppError> {
        let price_guides: PriceGuides = self
            .client
            .get(self.url.as_str())
            .send()
            .await?
            .json()
            .await?;

        Ok(price_guides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn should_fetch_and_deserialize_price_guides() {
        // Arrange
        let mock_server = MockServer::start().await;

        let json_body = r#"
        {
          "version": 1,
          "createdAt": "2025-12-23T02:47:26+0100",
          "priceGuides": [
            {
              "idProduct": 1,
              "idCategory": 1,
              "avg": 0.06,
              "low": 0.02,
              "trend": 0.09,
              "avg1": 0.1,
              "avg7": 0.06,
              "avg30": 0.07,
              "avg-foil": 0.5,
              "low-foil": 0.04,
              "trend-foil": 0.42,
              "avg1-foil": 0.5,
              "avg7-foil": 0.41,
              "avg30-foil": 0.34
            },
            {
              "idProduct": 2,
              "idCategory": 1,
              "avg": 0.06,
              "low": 0.02,
              "trend": 0.07,
              "avg1": 0.3,
              "avg7": 0.08,
              "avg30": 0.06,
              "avg-foil": null,
              "low-foil": 0.05,
              "trend-foil": 0.28,
              "avg1-foil": 0.49,
              "avg7-foil": 0.31,
              "avg30-foil": 0.25
            }
          ]
        }
        "#;

        Mock::given(method("GET"))
            .and(path("/price_guide.json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(json_body, "application/json"))
            .mount(&mock_server)
            .await;

        let adapter =
            CardMarketCallerAdapter::new(format!("{}/price_guide.json", mock_server.uri()));

        // Act
        let result = adapter.get_price_guides().await;

        // Assert
        assert!(result.is_ok());

        let price_guides = result.unwrap();
        // Assertions racine
        assert_eq!(
            price_guides
                .created_at
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-12-23 01:47:26"
        );

        // Assertions collection
        assert_eq!(price_guides.price_guides.len(), 2);

        let first = &price_guides.price_guides[0];
        assert_eq!(first.id_product, 1);
        assert_eq!(first.avg, Some(0.06));
        assert_eq!(first.low, Some(0.02));
        assert_eq!(first.trend, Some(0.09));
        assert_eq!(first.avg1, Some(0.1));
        assert_eq!(first.avg7, Some(0.06));
        assert_eq!(first.avg30, Some(0.07));
        assert_eq!(first.avg_foil, Some(0.5));
        assert_eq!(first.low_foil, Some(0.04));
        assert_eq!(first.trend_foil, Some(0.42));
        assert_eq!(first.avg1_foil, Some(0.5));
        assert_eq!(first.avg7_foil, Some(0.41));
        assert_eq!(first.avg30_foil, Some(0.34));

        let second = &price_guides.price_guides[1];
        assert_eq!(second.id_product, 2);
        assert_eq!(second.avg_foil, None);
        assert_eq!(second.trend, Some(0.07));
    }

    #[tokio::test]
    async fn should_return_error_when_http_fails() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let adapter = CardMarketCallerAdapter::new(mock_server.uri());

        let result = adapter.get_price_guides().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_fail_when_json_is_invalid() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw("this is not json", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let adapter = CardMarketCallerAdapter::new(mock_server.uri());

        let result = adapter.get_price_guides().await;

        assert!(result.is_err());
    }
}
