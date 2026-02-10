use crate::application::error::AppError;
use crate::application::repository::CardMarketRepository;
use crate::domain::price::FullPriceGuide;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

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
        id_produit: u32,
        price_guides: FullPriceGuide,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
            INSERT INTO cardmarket_price (id_produit, date, low, trend, avg, avg1, avg7, avg30, low_foil, trend_foil, avg_foil,
                              avg1_foil, avg7_foil, avg30_foil)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT(id_produit, date)
                DO UPDATE
                SET low        = $3,
                    trend      = $4,
                    avg        = $5,
                    avg1       = $6,
                    avg7       = $7,
                    avg30      = $8,
                    low_foil   = $9,
                    trend_foil = $10,
                    avg_foil   = $11,
                    avg1_foil  = $12,
                    avg7_foil  = $13,
                    avg30_foil = $14
            ",
            id_produit as i32,
            date,
            price_guides.normal.low.as_cents(),
            price_guides.normal.trend.as_cents(),
            price_guides.normal.avg.as_cents(),
            price_guides.normal.avg1.as_cents(),
            price_guides.normal.avg7.as_cents(),
            price_guides.normal.avg30.as_cents(),
            price_guides.foil.low.as_cents(),
            price_guides.foil.trend.as_cents(),
            price_guides.foil.avg.as_cents(),
            price_guides.foil.avg1.as_cents(),
            price_guides.foil.avg7.as_cents(),
            price_guides.foil.avg30.as_cents(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::price::{Price, PriceGuide};
    use chrono::NaiveDate;

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

        let result = repository.save(date, id_produit, price_guides).await;

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
            .save(date, id_produit, initial_price_guides)
            .await
            .unwrap();

        let updated_price_guides = create_full_price_guide(
            id_produit,
            (110, 160, 135, 130, 140, 150),
            (210, 260, 235, 230, 240, 250),
        );
        let result = repository
            .save(date, id_produit, updated_price_guides)
            .await;

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

        let result = repository.save(date, id_produit, price_guides).await;

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

        let result1 = repository.save(date, 12348, price_guides_1).await;
        let result2 = repository.save(date, 12349, price_guides_2).await;

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
