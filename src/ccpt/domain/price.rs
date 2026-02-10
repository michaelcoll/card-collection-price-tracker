use std::ops::AddAssign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Price {
    pub value: Option<u32>,
}

impl Price {
    pub fn empty() -> Self {
        Self { value: None }
    }

    pub(crate) fn from_cents(p0: u32) -> Price {
        Price { value: Some(p0) }
    }

    pub fn as_cents(&self) -> Option<i32> {
        self.value.map(|v| v as i32)
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

pub struct FullPriceGuide {
    pub id_product: u32,
    pub normal: PriceGuide,
    pub foil: PriceGuide,
}

impl PriceGuide {
    pub fn empty() -> Self {
        Self {
            low: Price::empty(),
            trend: Price::empty(),
            avg: Price::empty(),
            avg1: Price::empty(),
            avg7: Price::empty(),
            avg30: Price::empty(),
        }
    }

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
mod tests {
    use super::*;

    impl From<u32> for Price {
        fn from(value: u32) -> Self {
            Price { value: Some(value) }
        }
    }

    fn create_price(
        low: impl Into<Price>,
        trend: impl Into<Price>,
        avg: impl Into<Price>,
        avg1: impl Into<Price>,
        avg7: impl Into<Price>,
        avg30: impl Into<Price>,
    ) -> PriceGuide {
        PriceGuide {
            low: low.into(),
            trend: trend.into(),
            avg: avg.into(),
            avg1: avg1.into(),
            avg7: avg7.into(),
            avg30: avg30.into(),
        }
    }

    #[test]
    fn add_assign_combines_prices_correctly() {
        let mut price1 = create_price(100, 200, 200, 300, 400, 500);
        let price2 = create_price(50, 100, 100, 150, 200, 250);

        price1 += price2;

        assert_eq!(price1, create_price(150, 300, 300, 450, 600, 750));
    }

    #[test]
    fn add_assign_with_zero_price_does_not_change_values() {
        let mut price1 = PriceGuide::empty();
        let price2 = create_price(100, 200, 200, 300, 400, 500);

        price1 += price2;

        assert_eq!(price1, create_price(100, 200, 200, 300, 400, 500));
    }
}
