use crate::application::caller::EdhRecCaller;
use crate::application::error::AppError;
use crate::domain::card::CardInfo;
use crate::infrastructure::adapter_out::caller::dto::EdhRecCardInfo;
use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime, Utc};
use std::option::Option;
use tokio::sync::RwLock;

struct BuildIdCache {
    id: Option<String>,
    last_updated: Option<NaiveDateTime>,
}

pub struct EdhRecCallerAdapter {
    client: reqwest::Client,
    edh_rec_base_url: String,
    cache: RwLock<BuildIdCache>,
}

impl EdhRecCallerAdapter {
    pub fn new(edh_rec_base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            edh_rec_base_url: edh_rec_base_url.into(),
            cache: RwLock::new(BuildIdCache {
                id: None,
                last_updated: None,
            }),
        }
    }

    async fn get_build_id(&self) -> Result<String, AppError> {
        let url = self.edh_rec_base_url.clone() + "/faq";
        println!("Fetching build ID from {}", url);

        let html = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::CallError(format!("edhrec request error: {e}")))?
            .text()
            .await
            .map_err(|e| AppError::CallError(format!("edhrec response read error: {e}")))?;

        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse(r#"script#__NEXT_DATA__"#)
            .map_err(|e| AppError::CallError(format!("invalid selector: {e}")))?;

        let json_text = document
            .select(&selector)
            .next()
            .and_then(|el| el.text().next())
            .ok_or_else(|| {
                AppError::CallError("unable to find __NEXT_DATA__ script".to_string())
            })?;

        let v: serde_json::Value = serde_json::from_str(json_text)
            .map_err(|e| AppError::CallError(format!("__NEXT_DATA__ is not valid json: {e}")))?;

        let build_id = v
            .get("buildId")
            .and_then(|x| x.as_str())
            .ok_or_else(|| AppError::CallError("buildId not found in __NEXT_DATA__".to_string()))?
            .to_string();

        println!("Build ID: {build_id}");

        Ok(build_id)
    }

    async fn update_build_id(&self) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        {
            let cache = self.cache.read().await;
            if let Some(last) = cache.last_updated
                && (now - last) < Duration::hours(24)
            {
                return Ok(());
            }
        }

        let new_id = self.get_build_id().await?;

        {
            let mut cache = self.cache.write().await;
            cache.id = Some(new_id.clone());
            cache.last_updated = Some(now);
        }

        Ok(())
    }

    fn get_card_id_from_name(&self, name: &str) -> String {
        name.replace(' ', "-")
            .replace("'", "")
            .replace(",", "")
            .to_lowercase()
    }
}

#[async_trait]
impl EdhRecCaller for EdhRecCallerAdapter {
    async fn get_card_info(&self, card_name: String) -> Result<CardInfo, AppError> {
        self.update_build_id().await?;

        let url = format!(
            "{}/_next/data/{}/cards/{}.json",
            self.edh_rec_base_url,
            self.cache.read().await.id.as_ref().unwrap(),
            self.get_card_id_from_name(&card_name)
        );

        println!("Fetching cardinfo from {}", url);

        let card_info: EdhRecCardInfo = self.client.get(url).send().await?.json().await?;

        let edh_rec_card = card_info.page_props.data.container.json_dict.card;

        println!("edh rec card info: {:?}", edh_rec_card);

        Ok(CardInfo {
            inclusion: edh_rec_card.inclusion as u32,
            total_decks: edh_rec_card.potential_decks as u32,
        })
    }
}

#[cfg(test)]
#[path = "edhrec_caller_adapter_tests.rs"]
mod tests;
