use crate::domain::card::{Card, CollectionEntry};
use crate::domain::collection::{CollectionSortField, SortDirection};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

// --- Query params ---
#[derive(Deserialize, Default, TS, ToSchema)]
#[serde(rename = "SortBy", rename_all = "snake_case")]
#[ts(export, export_to = "SortBy.ts")]
pub enum SortByParam {
    Avg,
    #[default]
    Trend,
    SetCode,
    LanguageCode,
}

#[derive(Deserialize, Default, TS, ToSchema)]
#[serde(rename = "SortDir", rename_all = "snake_case")]
#[ts(export, export_to = "SortDir.ts")]
pub enum SortDirParam {
    Asc,
    #[default]
    Desc,
}

impl From<SortByParam> for CollectionSortField {
    fn from(p: SortByParam) -> Self {
        match p {
            SortByParam::Avg => CollectionSortField::Avg,
            SortByParam::Trend => CollectionSortField::Trend,
            SortByParam::SetCode => CollectionSortField::SetCode,
            SortByParam::LanguageCode => CollectionSortField::LanguageCode,
        }
    }
}

impl From<SortDirParam> for SortDirection {
    fn from(p: SortDirParam) -> Self {
        match p {
            SortDirParam::Asc => SortDirection::Asc,
            SortDirParam::Desc => SortDirection::Desc,
        }
    }
}

pub(crate) fn default_page_size() -> u32 {
    20
}

pub(crate) fn max_page_size() -> u32 {
    100
}

#[derive(Deserialize)]
pub(crate) struct CollectionParams {
    #[serde(default)]
    pub(crate) page: u32,
    #[serde(default = "default_page_size")]
    pub(crate) page_size: u32,
    #[serde(default)]
    pub(crate) sort_by: SortByParam,
    #[serde(default)]
    pub(crate) sort_dir: SortDirParam,
    pub(crate) q: Option<String>,
    /// Comma-separated rarity codes (C, U, R, M)
    pub(crate) rarity: Option<String>,
    /// Comma-separated set codes
    pub(crate) sets: Option<String>,
    /// Minimum trend price in cents
    pub(crate) price_min: Option<u32>,
    /// Maximum trend price in cents
    pub(crate) price_max: Option<u32>,
    /// Restrict to cards owned by the authenticated user (default: false — full catalog)
    #[serde(default)]
    pub(crate) owned: bool,
}

impl Default for CollectionParams {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: default_page_size(),
            sort_by: SortByParam::default(),
            sort_dir: SortDirParam::default(),
            q: None,
            rarity: None,
            sets: None,
            price_min: None,
            price_max: None,
            owned: false,
        }
    }
}

// --- Réponses ---
#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "PriceGuide")]
#[ts(export, export_to = "PriceGuide.ts")]
pub struct PriceGuideResponse {
    pub low: Option<u32>,
    pub avg: Option<u32>,
    pub trend: Option<u32>,
}

#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "CollectionEntry")]
#[ts(export, export_to = "CollectionEntry.ts")]
pub struct CollectionEntryResponse {
    pub quantity: u8,
    pub purchase_price: u32,
    /// RFC 3339 timestamp
    pub added_at: String,
}

#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "CollectionCard")]
#[ts(export, export_to = "CollectionCard.ts")]
pub struct CollectionCardResponse {
    pub set_code: String,
    pub collector_number: String,
    pub language_code: String,
    pub foil: bool,
    pub name: String,
    pub rarity_code: String,
    pub scryfall_id: String,
    pub the_gatherer_id: Option<String>,
    /// Present only when the card is owned by the authenticated user.
    pub collection_entry: Option<CollectionEntryResponse>,
    /// Username of the owner when the card belongs to another user (catalog listing).
    pub owner_username: Option<String>,
    pub price_guide: Option<PriceGuideResponse>,
}

#[derive(Serialize, TS, ToSchema)]
#[serde(rename = "PaginatedCollection")]
#[ts(export, export_to = "PaginatedCollection.ts")]
pub struct PaginatedCollectionResponse {
    pub items: Vec<CollectionCardResponse>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

impl From<Card> for CollectionCardResponse {
    fn from(c: Card) -> Self {
        let (collection_entry, owner_username) = match c.collection_entry {
            CollectionEntry::Mine {
                quantity,
                purchase_price,
                added_at,
            } => (
                Some(CollectionEntryResponse {
                    quantity,
                    purchase_price,
                    added_at: added_at.to_rfc3339(),
                }),
                None,
            ),
            CollectionEntry::Owned { owner_username } => (None, Some(owner_username)),
        };

        Self {
            set_code: c.id.set_code.to_string(),
            collector_number: c.id.collector_number,
            language_code: c.id.language_code.to_string(),
            foil: c.id.foil,
            name: c.name,
            rarity_code: c.rarity_code.to_string(),
            scryfall_id: c.scryfall_id.to_string(),
            the_gatherer_id: c.the_gatherer_id,
            collection_entry,
            owner_username,
            price_guide: c.price_guide.map(|pg| PriceGuideResponse {
                low: pg.low.value,
                avg: pg.avg.value,
                trend: pg.trend.value,
            }),
        }
    }
}
