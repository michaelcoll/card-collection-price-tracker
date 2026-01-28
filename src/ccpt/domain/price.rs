use chrono::NaiveDate;
use std::ops::AddAssign;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Price {
    pub date: NaiveDate,
    /// Low price in cents
    pub low: u32,
    /// Trend price in cents
    pub trend: u32,
    /// Average price for 1 day in cents
    pub avg1: u32,
    /// Average price for 7 days in cents
    pub avg7: u32,
    /// Average price for 30 days in cents
    pub avg30: u32,
}

impl Price {
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self {
            date: Default::default(),
            low: 0,
            trend: 0,
            avg1: 0,
            avg7: 0,
            avg30: 0,
        }
    }
}

impl AddAssign for Price {
    fn add_assign(&mut self, other: Self) {
        self.date = other.date;
        self.low += other.low;
        self.trend += other.trend;
        self.avg1 += other.avg1;
        self.avg7 += other.avg7;
        self.avg30 += other.avg30;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_price(date: &str, low: u32, trend: u32, avg1: u32, avg7: u32, avg30: u32) -> Price {
        Price {
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            low,
            trend,
            avg1,
            avg7,
            avg30,
        }
    }

    #[test]
    fn add_assign_combines_prices_correctly() {
        let mut price1 = create_price("2023-10-01", 100, 200, 300, 400, 500);
        let price2 = create_price("2023-10-02", 50, 100, 150, 200, 250);

        price1 += price2;

        assert_eq!(price1, create_price("2023-10-02", 150, 300, 450, 600, 750));
    }

    #[test]
    fn add_assign_with_zero_price_does_not_change_values() {
        let mut price1 = Price::zero();
        let price2 = create_price("2023-10-01", 100, 200, 300, 400, 500);

        price1 += price2;

        assert_eq!(price1, create_price("2023-10-01", 100, 200, 300, 400, 500));
    }

    #[test]
    fn zero_creates_price_with_all_zero_values() {
        let zero_price = Price::zero();

        assert_eq!(zero_price, create_price("1970-01-01", 0, 0, 0, 0, 0));
    }
}
