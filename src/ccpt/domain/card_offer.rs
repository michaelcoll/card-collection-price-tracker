use crate::domain::card::CollectionEntry;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum CardOfferSortField {
    #[default]
    SellingPrice,
}

#[derive(Clone, Debug)]
pub struct PaginatedCardOffers {
    pub items: Vec<CollectionEntry>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}
