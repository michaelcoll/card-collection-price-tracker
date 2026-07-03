use crate::infrastructure::adapter_out::repository::entities::{
    CardMarketPriceEntity, CardMarketPriceRaw, CollectionPriceHistoryEntity,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_set(pool: &PgPool, set_code: &str) {
    sqlx::query(
        r#"INSERT INTO set_name (set_code, name)
             VALUES ($1, $2)"#,
    )
    .bind(set_code)
    .bind(format!("Set {}", set_code))
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_card(
    pool: &PgPool,
    set_code: &str,
    collector_number: &str,
    language_code: &str,
    foil: bool,
    name: &str,
    cardmarket_id: i32,
) {
    sqlx::query(
        r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
        .bind(set_code)
        .bind(collector_number)
        .bind(language_code)
        .bind(foil)
        .bind(name)
        .bind("C")
        .bind(Uuid::new_v4())
        .bind(cardmarket_id)
    .execute(pool)
    .await
    .unwrap();
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_card_with_rarity(
    pool: &PgPool,
    set_code: &str,
    collector_number: &str,
    language_code: &str,
    foil: bool,
    name: &str,
    cardmarket_id: i32,
    rarity: &str,
) {
    sqlx::query(
        r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
        .bind(set_code)
        .bind(collector_number)
        .bind(language_code)
        .bind(foil)
        .bind(name)
        .bind(rarity)
        .bind(Uuid::new_v4())
        .bind(cardmarket_id)
    .execute(pool)
    .await
    .unwrap();
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_collection_entry(
    pool: &PgPool,
    set_code: &str,
    collector_number: &str,
    language_code: &str,
    foil: bool,
    user_id: &str,
    quantity: i32,
    purchase_price: i32,
    date: chrono::DateTime<chrono::Utc>,
) {
    sqlx::query(
        r#"INSERT INTO collection_entry (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price, added_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
        .bind(set_code)
        .bind(collector_number)
        .bind(language_code)
        .bind(foil)
        .bind(user_id)
        .bind(quantity)
        .bind(purchase_price)
        .bind(date)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_card_without_cardmarket_id(
    pool: &PgPool,
    set_code: &str,
    collector_number: &str,
    language_code: &str,
    foil: bool,
    name: &str,
) {
    sqlx::query(
        r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind(set_code)
    .bind(collector_number)
    .bind(language_code)
    .bind(foil)
    .bind(name)
    .bind("C")
    .bind(Uuid::new_v4())
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_price(pool: &PgPool, entity: CardMarketPriceEntity) {
    sqlx::query(
        r#"INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30, low_foil, avg_foil, trend_foil, avg1_foil, avg7_foil, avg30_foil)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
    )
    .bind(entity.id_produit)
    .bind(entity.date)
    .bind(entity.normal.low)
    .bind(entity.normal.avg)
    .bind(entity.normal.trend)
    .bind(entity.normal.avg1)
    .bind(entity.normal.avg7)
    .bind(entity.normal.avg30)
    .bind(entity.foil.low)
    .bind(entity.foil.avg)
    .bind(entity.foil.trend)
    .bind(entity.foil.avg1)
    .bind(entity.foil.avg7)
    .bind(entity.foil.avg30)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn fetch_collection_price_history(
    pool: &PgPool,
    date: chrono::NaiveDate,
    user_id: &str,
) -> Vec<CollectionPriceHistoryEntity> {
    sqlx::query_as!(
        CollectionPriceHistoryEntity,
        r#"SELECT date, low, trend, avg, avg1, avg7, avg30
                FROM collection_price_history
                WHERE user_id = $1
                  AND date >= $2
                  AND date <= $3
                ORDER BY date"#,
        user_id,
        date,
        date,
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn fetch_cardmarket_price(
    pool: &PgPool,
    id_produit: i32,
    date: chrono::NaiveDate,
) -> CardMarketPriceEntity {
    sqlx::query_as!(
        CardMarketPriceRaw,
        "SELECT id_produit, date, low, trend, avg, avg1, avg7, avg30,
                    low_foil, trend_foil, avg_foil, avg1_foil, avg7_foil, avg30_foil
             FROM cardmarket_price
             WHERE id_produit = $1 AND date = $2",
        id_produit,
        date
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .into()
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_collection_price_history(
    pool: &PgPool,
    date: chrono::NaiveDate,
    user_id: &str,
    low: i32,
    avg: i32,
    trend: i32,
    avg1: i32,
    avg7: i32,
    avg30: i32,
) {
    sqlx::query(
        r#"INSERT INTO collection_price_history (date, user_id, low, avg, trend, avg1, avg7, avg30)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
    )
    .bind(date)
    .bind(user_id)
    .bind(low)
    .bind(avg)
    .bind(trend)
    .bind(avg1)
    .bind(avg7)
    .bind(avg30)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn refresh_view(pool: &PgPool) {
    sqlx::query("REFRESH MATERIALIZED VIEW mv_card_prices")
        .execute(pool)
        .await
        .unwrap();
}
