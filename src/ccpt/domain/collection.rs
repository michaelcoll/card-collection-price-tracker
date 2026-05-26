use crate::domain::card::Card;
use std::fmt;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum CollectionSortField {
    Avg,
    #[default]
    Trend,
    SetCode,
    LanguageCode,
}

impl fmt::Display for CollectionSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Avg => write!(f, "avg"),
            Self::Trend => write!(f, "trend"),
            Self::SetCode => write!(f, "set_code"),
            Self::LanguageCode => write!(f, "language_code"),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    #[default]
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CollectionQuery {
    pub page: u32,
    pub page_size: u32,
    pub sort_by: CollectionSortField,
    pub sort_dir: SortDirection,
    pub search_query: Option<String>,
}

impl Default for CollectionQuery {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 20,
            sort_by: CollectionSortField::default(),
            sort_dir: SortDirection::default(),
            search_query: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PaginatedCollection {
    pub items: Vec<Card>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

#[cfg(test)]
#[path = "collection_tests.rs"]
mod tests;
