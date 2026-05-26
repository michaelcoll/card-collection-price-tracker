use crate::application::caller::CardMarketCaller;
use crate::application::error::AppError;
use crate::domain::price::FullPriceGuide;
use crate::infrastructure::adapter_out::caller::dto::CardmarketPriceGuides;
use async_trait::async_trait;
use chrono::NaiveDate;

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
    async fn get_price_guides(&self) -> Result<(NaiveDate, Vec<FullPriceGuide>), AppError> {
        let price_guides: CardmarketPriceGuides = self
            .client
            .get(self.url.as_str())
            .send()
            .await?
            .json()
            .await?;

        let domain = price_guides
            .price_guides
            .into_iter()
            .map(|pg| pg.into())
            .collect();

        Ok((price_guides.created_at.date_naive(), domain))
    }
}

#[cfg(test)]
#[path = "cardmarket_caller_adapter_tests.rs"]
mod tests;
