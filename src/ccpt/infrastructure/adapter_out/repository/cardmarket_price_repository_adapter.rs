use crate::application::error::AppError;
use crate::application::repository::CardMarketPriceRepository;
use crate::domain::price::FullPriceGuide;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres, QueryBuilder};

pub struct CardMarketPriceRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardMarketPriceRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardMarketPriceRepository for CardMarketPriceRepositoryAdapter {
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
#[path = "cardmarket_price_repository_adapter_tests.rs"]
mod tests;
