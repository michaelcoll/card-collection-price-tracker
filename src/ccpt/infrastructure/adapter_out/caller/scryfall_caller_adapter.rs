use crate::application::caller::ScryfallCaller;
use crate::application::error::AppError;
use crate::infrastructure::adapter_out::caller::dto::ScryfallCardInfo;
use async_trait::async_trait;
use ratelimit::{Ratelimiter, TryWaitError};
use uuid::Uuid;

pub struct ScryfallCallerAdapter {
    client: reqwest::Client,
    scryfall_base_url: String,
    ratelimiter: Ratelimiter,
}

impl ScryfallCallerAdapter {
    pub fn new(scryfall_base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("reqwest")
                .build()
                .unwrap(),
            scryfall_base_url: scryfall_base_url.into(),
            ratelimiter: Ratelimiter::builder(8).max_tokens(8).build().unwrap(),
        }
    }
}

#[async_trait]
impl ScryfallCaller for ScryfallCallerAdapter {
    async fn get_card_market_id(&self, id: Uuid) -> Result<Option<u32>, AppError> {
        let url = format!("{}/cards/{}?format=json", self.scryfall_base_url, id);

        if let Err(err) = self.ratelimiter.try_wait() {
            match err {
                TryWaitError::Insufficient(duration) => {
                    tokio::time::sleep(duration).await;
                }
                TryWaitError::ExceedsCapacity => {
                    return Err(AppError::CallError(
                        "Scryfall rate limiter overflow".to_string(),
                    ));
                }
                _ => {
                    return Err(AppError::CallError(
                        "Scryfall rate limiter error".to_string(),
                    ));
                }
            }
        }

        let card_info: ScryfallCardInfo =
            self.client.get(url.as_str()).send().await?.json().await?;

        Ok(card_info.cardmarket_id.map(|id| id as u32))
    }
}

#[cfg(test)]
#[path = "scryfall_caller_adapter_tests.rs"]
mod tests;
