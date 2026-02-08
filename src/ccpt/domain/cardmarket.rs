use chrono::{DateTime, Utc};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceGuides {
    pub created_at: DateTime<Utc>,
    pub price_guides: Vec<PriceGuide>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PriceGuide {
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
