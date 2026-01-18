use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionPriceHistory {
    pub date: NaiveDate,
    /// Low price in cents
    pub low: i32,
    /// Trend price in cents
    pub trend: i32,
    /// Average price for 1 day in cents
    pub avg1: i32,
    /// Average price for 7 days in cents
    pub avg7: i32,
    /// Average price for 30 days in cents
    pub avg30: i32,
}
