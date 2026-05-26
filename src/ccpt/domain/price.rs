use chrono::NaiveDate;
use std::ops::AddAssign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Price {
    pub value: Option<u32>,
}

impl Price {
    pub fn empty() -> Self {
        Self { value: None }
    }
}

impl AddAssign for Price {
    fn add_assign(&mut self, other: Self) {
        self.value = match (self.value, other.value) {
            (Some(a), Some(b)) => Some(a + b),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriceGuide {
    /// Low price in cents
    pub low: Price,
    /// Average price in cents
    pub avg: Price,
    /// Trend price in cents
    pub trend: Price,
    /// Average price for 1 day in cents
    pub avg1: Price,
    /// Average price for 7 days in cents
    pub avg7: Price,
    /// Average price for 30 days in cents
    pub avg30: Price,
}

/// Represents one day's aggregated price for a user's collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriceHistoryEntry {
    pub date: NaiveDate,
    pub price_guide: PriceGuide,
}

pub struct FullPriceGuide {
    pub id_product: u32,
    pub normal: PriceGuide,
    pub foil: PriceGuide,
}

impl PriceGuide {
    pub fn new(
        low: impl Into<Price>,
        trend: impl Into<Price>,
        avg: impl Into<Price>,
        avg1: impl Into<Price>,
        avg7: impl Into<Price>,
        avg30: impl Into<Price>,
    ) -> Self {
        Self {
            low: low.into(),
            trend: trend.into(),
            avg: avg.into(),
            avg1: avg1.into(),
            avg7: avg7.into(),
            avg30: avg30.into(),
        }
    }
}

impl AddAssign for PriceGuide {
    fn add_assign(&mut self, other: Self) {
        self.low += other.low;
        self.trend += other.trend;
        self.avg += other.avg;
        self.avg1 += other.avg1;
        self.avg7 += other.avg7;
        self.avg30 += other.avg30;
    }
}

#[cfg(test)]
#[path = "price_tests.rs"]
mod tests;
