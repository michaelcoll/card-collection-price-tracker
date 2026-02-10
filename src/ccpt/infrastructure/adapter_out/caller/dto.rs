use crate::domain::price::{FullPriceGuide, Price, PriceGuide};
use chrono::{DateTime, Utc};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardmarketPriceGuides {
    pub created_at: DateTime<Utc>,
    pub price_guides: Vec<CardmarketPriceGuide>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CardmarketPriceGuide {
    #[serde(rename(deserialize = "idProduct"))]
    pub id_product: u32,
    pub avg: Option<f32>,
    pub low: Option<f32>,
    pub trend: Option<f32>,
    pub avg1: Option<f32>,
    pub avg7: Option<f32>,
    pub avg30: Option<f32>,
    #[serde(rename(deserialize = "avg-foil"))]
    pub avg_foil: Option<f32>,
    #[serde(rename(deserialize = "low-foil"))]
    pub low_foil: Option<f32>,
    #[serde(rename(deserialize = "trend-foil"))]
    pub trend_foil: Option<f32>,
    #[serde(rename(deserialize = "avg1-foil"))]
    pub avg1_foil: Option<f32>,
    #[serde(rename(deserialize = "avg7-foil"))]
    pub avg7_foil: Option<f32>,
    #[serde(rename(deserialize = "avg30-foil"))]
    pub avg30_foil: Option<f32>,
}

impl From<Option<f32>> for Price {
    fn from(value: Option<f32>) -> Self {
        value
            .map(|v| (v * 100.0).round() as u32)
            .map(Price::from_cents)
            .unwrap_or_else(Price::empty)
    }
}

impl From<CardmarketPriceGuide> for FullPriceGuide {
    fn from(value: CardmarketPriceGuide) -> Self {
        FullPriceGuide {
            id_product: value.id_product,
            normal: PriceGuide::new(
                value.low,
                value.trend,
                value.avg,
                value.avg1,
                value.avg7,
                value.avg30,
            ),
            foil: PriceGuide::new(
                value.low_foil,
                value.trend_foil,
                value.avg_foil,
                value.avg1_foil,
                value.avg7_foil,
                value.avg30_foil,
            ),
        }
    }
}
