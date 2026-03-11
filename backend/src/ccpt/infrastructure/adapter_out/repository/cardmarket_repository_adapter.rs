use crate::application::error::AppError;
use crate::application::repository::CardMarketRepository;
use crate::domain::price::FullPriceGuide;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres, QueryBuilder};

pub struct CardMarketRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardMarketRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardMarketRepository for CardMarketRepositoryAdapter {
    async fn save(
        &self,
        date: NaiveDate,
        price_guides: Vec<FullPriceGuide>,
    ) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        const CHUNK_SIZE: usize = 1000;

        for chunk in price_guides.chunks(CHUNK_SIZE) {
            let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
                "
                INSERT INTO cardmarket_price
                    (id_produit, date, low, trend, avg, avg1, avg7, avg30,
                     low_foil, trend_foil, avg_foil, avg1_foil, avg7_foil, avg30_foil)
                ",
            );

            qb.push_values(chunk, |mut b, price_guide| {
                b.push_bind(price_guide.id_product as i32)
                    .push_bind(date)
                    .push_bind(price_guide.normal.low.as_i32())
                    .push_bind(price_guide.normal.trend.as_i32())
                    .push_bind(price_guide.normal.avg.as_i32())
                    .push_bind(price_guide.normal.avg1.as_i32())
                    .push_bind(price_guide.normal.avg7.as_i32())
                    .push_bind(price_guide.normal.avg30.as_i32())
                    .push_bind(price_guide.foil.low.as_i32())
                    .push_bind(price_guide.foil.trend.as_i32())
                    .push_bind(price_guide.foil.avg.as_i32())
                    .push_bind(price_guide.foil.avg1.as_i32())
                    .push_bind(price_guide.foil.avg7.as_i32())
                    .push_bind(price_guide.foil.avg30.as_i32());
            });

            qb.push(
                "
                ON CONFLICT (id_produit, date)
                DO UPDATE SET
                    low        = EXCLUDED.low,
                    trend      = EXCLUDED.trend,
                    avg        = EXCLUDED.avg,
                    avg1       = EXCLUDED.avg1,
                    avg7       = EXCLUDED.avg7,
                    avg30      = EXCLUDED.avg30,
                    low_foil   = EXCLUDED.low_foil,
                    trend_foil = EXCLUDED.trend_foil,
                    avg_foil   = EXCLUDED.avg_foil,
                    avg1_foil  = EXCLUDED.avg1_foil,
                    avg7_foil  = EXCLUDED.avg7_foil,
                    avg30_foil = EXCLUDED.avg30_foil
                ",
            );

            qb.build().execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::price::{Price, PriceGuide};
    use chrono::NaiveDate;

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
    }

    fn create_full_price_guide(
        id_product: u32,
        normal_values: (u32, u32, u32, u32, u32, u32),
        foil_values: (u32, u32, u32, u32, u32, u32),
    ) -> FullPriceGuide {
        FullPriceGuide {
            id_product,
            normal: PriceGuide {
                low: Price::from_cents(normal_values.0),
                trend: Price::from_cents(normal_values.1),
                avg: Price::from_cents(normal_values.2),
                avg1: Price::from_cents(normal_values.3),
                avg7: Price::from_cents(normal_values.4),
                avg30: Price::from_cents(normal_values.5),
            },
            foil: PriceGuide {
                low: Price::from_cents(foil_values.0),
                trend: Price::from_cents(foil_values.1),
                avg: Price::from_cents(foil_values.2),
                avg1: Price::from_cents(foil_values.3),
                avg7: Price::from_cents(foil_values.4),
                avg30: Price::from_cents(foil_values.5),
            },
        }
    }

    #[sqlx::test]
    async fn test_save_new_cardmarket_price(pool: Pool<Postgres>) {
        let repository = CardMarketRepositoryAdapter::new(pool.clone());
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let id_produit = 12345u32;
        let price_guides = create_full_price_guide(
            id_produit,
            (100, 150, 125, 120, 130, 140),
            (200, 250, 225, 220, 230, 240),
        );

        let result = repository.save(date, vec![price_guides]).await;

        assert!(result.is_ok());

        let record = sqlx::query!(
            "SELECT id_produit, date, low, trend, avg, avg1, avg7, avg30, 
                    low_foil, trend_foil, avg_foil, avg1_foil, avg7_foil, avg30_foil 
             FROM cardmarket_price WHERE id_produit = $1 AND date = $2",
            id_produit as i32,
            date
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(record.id_produit, id_produit as i32);
        assert_eq!(record.date, date);
        assert_eq!(record.low, Some(100));
        assert_eq!(record.trend, Some(150));
        assert_eq!(record.avg, Some(125));
        assert_eq!(record.avg1, Some(120));
        assert_eq!(record.avg7, Some(130));
        assert_eq!(record.avg30, Some(140));
        assert_eq!(record.low_foil, Some(200));
        assert_eq!(record.trend_foil, Some(250));
        assert_eq!(record.avg_foil, Some(225));
        assert_eq!(record.avg1_foil, Some(220));
        assert_eq!(record.avg7_foil, Some(230));
        assert_eq!(record.avg30_foil, Some(240));
    }

    #[sqlx::test]
    async fn test_save_updates_existing_cardmarket_price(pool: Pool<Postgres>) {
        let repository = CardMarketRepositoryAdapter::new(pool.clone());
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let id_produit = 12346u32;

        let initial_price_guides = create_full_price_guide(
            id_produit,
            (100, 150, 125, 120, 130, 140),
            (200, 250, 225, 220, 230, 240),
        );
        repository
            .save(date, vec![initial_price_guides])
            .await
            .unwrap();

        let updated_price_guides = create_full_price_guide(
            id_produit,
            (110, 160, 135, 130, 140, 150),
            (210, 260, 235, 230, 240, 250),
        );
        let result = repository.save(date, vec![updated_price_guides]).await;

        assert!(result.is_ok());

        let record = sqlx::query!(
            "SELECT id_produit, date, low, trend, avg, avg1, avg7, avg30, 
                    low_foil, trend_foil, avg_foil, avg1_foil, avg7_foil, avg30_foil 
             FROM cardmarket_price WHERE id_produit = $1 AND date = $2",
            id_produit as i32,
            date
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(record.low, Some(110));
        assert_eq!(record.trend, Some(160));
        assert_eq!(record.avg, Some(135));
        assert_eq!(record.avg1, Some(130));
        assert_eq!(record.avg7, Some(140));
        assert_eq!(record.avg30, Some(150));
        assert_eq!(record.low_foil, Some(210));
        assert_eq!(record.trend_foil, Some(260));
        assert_eq!(record.avg_foil, Some(235));
        assert_eq!(record.avg1_foil, Some(230));
        assert_eq!(record.avg7_foil, Some(240));
        assert_eq!(record.avg30_foil, Some(250));
    }

    #[sqlx::test]
    async fn test_save_with_empty_price_values(pool: Pool<Postgres>) {
        let repository = CardMarketRepositoryAdapter::new(pool.clone());
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let id_produit = 12347u32;

        let price_guides = FullPriceGuide {
            id_product: id_produit,
            normal: PriceGuide::empty(),
            foil: PriceGuide::empty(),
        };

        let result = repository.save(date, vec![price_guides]).await;

        assert!(result.is_ok());

        let record = sqlx::query!(
            "SELECT id_produit, date, low, trend, avg, avg1, avg7, avg30, 
                    low_foil, trend_foil, avg_foil, avg1_foil, avg7_foil, avg30_foil 
             FROM cardmarket_price WHERE id_produit = $1 AND date = $2",
            id_produit as i32,
            date
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(record.id_produit, id_produit as i32);
        assert_eq!(record.date, date);
        assert_eq!(record.low, None);
        assert_eq!(record.trend, None);
        assert_eq!(record.avg, None);
        assert_eq!(record.avg1, None);
        assert_eq!(record.avg7, None);
        assert_eq!(record.avg30, None);
        assert_eq!(record.low_foil, None);
        assert_eq!(record.trend_foil, None);
        assert_eq!(record.avg_foil, None);
        assert_eq!(record.avg1_foil, None);
        assert_eq!(record.avg7_foil, None);
        assert_eq!(record.avg30_foil, None);
    }

    #[sqlx::test]
    async fn test_save_handles_multiple_products_same_date(pool: Pool<Postgres>) {
        let repository = CardMarketRepositoryAdapter::new(pool.clone());
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let price_guides_1 = create_full_price_guide(
            12348,
            (100, 150, 125, 120, 130, 140),
            (200, 250, 225, 220, 230, 240),
        );
        let price_guides_2 = create_full_price_guide(
            12349,
            (300, 350, 325, 320, 330, 340),
            (400, 450, 425, 420, 430, 440),
        );

        let result1 = repository.save(date, vec![price_guides_1]).await;
        let result2 = repository.save(date, vec![price_guides_2]).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM cardmarket_price WHERE date = $1")
                .bind(date)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(count, 2);
    }
}
