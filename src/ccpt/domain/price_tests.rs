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
    let mut price1 = PriceGuide::new(
        Price::empty(),
        Price::empty(),
        Price::empty(),
        Price::empty(),
        Price::empty(),
        Price::empty(),
    );
    let price2 = create_price(100, 200, 200, 300, 400, 500);

    price1 += price2;

    assert_eq!(price1, create_price(100, 200, 200, 300, 400, 500));
}
