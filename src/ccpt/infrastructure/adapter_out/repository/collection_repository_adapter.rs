use crate::application::error::AppError;
use crate::application::repository::CollectionRepository;
use crate::domain::card::Card;
use crate::domain::collection::{CollectionQuery, PaginatedCollection};
use crate::infrastructure::adapter_out::repository::entities::CardWithPriceEntity;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct CollectionRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CollectionRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionRepository for CollectionRepositoryAdapter {
    async fn get_paginated(
        &self,
        user_id: &str,
        query: CollectionQuery,
    ) -> Result<PaginatedCollection, AppError> {
        let sql = format!(
            r#"SELECT
                 p.set_code,
                 sn.name AS set_name,
                 p.collector_number,
                 p.language_code,
                 p.foil,
                 p.name,
                 p.rarity,
                 p.scryfall_id,
                 p.quantity,
                 p.purchase_price,
                 p.avg,
                 p.low,
                 p.trend,
                 p.avg1,
                 p.avg7,
                 p.avg30
               FROM mv_card_prices p
               JOIN set_name sn ON sn.set_code = p.set_code
               WHERE p.user_id = $1
               ORDER BY {} {} NULLS LAST
               LIMIT $2 OFFSET $3"#,
            query.sort_by, query.sort_dir,
        );

        let offset = (query.page * query.page_size) as i64;
        let limit = query.page_size as i64;

        let entities = sqlx::query_as::<_, CardWithPriceEntity>(&sql)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::RepositoryError(e.to_string()))?;

        let total: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM mv_card_prices WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| AppError::RepositoryError(e.to_string()))?;

        Ok(PaginatedCollection {
            items: entities.into_iter().map(Card::from).collect(),
            total: total as u64,
            page: query.page,
            page_size: query.page_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::repository::CollectionRepository;
    use crate::domain::collection::{CollectionQuery, CollectionSortField, SortDirection};
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn setup_card(pool: &PgPool, set_code: &str, collector_number: &str, cardmarket_id: i32) {
        sqlx::query!(
            r#"INSERT INTO set_name (set_code, name) VALUES ($1, $2) ON CONFLICT DO NOTHING"#,
            set_code,
            format!("Set {}", set_code)
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
               VALUES ($1, $2, 'EN', false, 'Test Card', 'C', $3, $4)
               ON CONFLICT DO NOTHING"#,
            set_code,
            collector_number,
            Uuid::new_v4(),
            cardmarket_id,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    async fn setup_card_quantity(
        pool: &PgPool,
        set_code: &str,
        collector_number: &str,
        user_id: &str,
        quantity: i32,
        purchase_price: i32,
    ) {
        sqlx::query!(
            r#"INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
               VALUES ($1, $2, 'EN', false, $3, $4, $5)"#,
            set_code,
            collector_number,
            user_id,
            quantity,
            purchase_price,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    async fn setup_price(pool: &PgPool, cardmarket_id: i32, avg: i32) {
        sqlx::query!(
            r#"INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
               VALUES ($1, CURRENT_DATE, $2, $3, $2, $2, $2, $2)"#,
            cardmarket_id,
            avg / 2,
            avg,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    async fn refresh_view(pool: &PgPool) {
        sqlx::query("REFRESH MATERIALIZED VIEW mv_card_prices")
            .execute(pool)
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn get_paginated_returns_empty_when_no_cards_in_collection(pool: PgPool) {
        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert!(result.items.is_empty());
        assert_eq!(result.total, 0);
    }

    #[sqlx::test]
    async fn get_paginated_returns_cards_for_the_given_user(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 2, 500).await;
        setup_price(&pool, 1, 200).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
    }

    #[sqlx::test]
    async fn get_paginated_does_not_return_cards_belonging_to_another_user(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user_other", 1, 100).await;
        setup_price(&pool, 1, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert!(result.items.is_empty());
        assert_eq!(result.total, 0);
    }

    #[sqlx::test]
    async fn get_paginated_respects_page_size(pool: PgPool) {
        for i in 1..=5i32 {
            let set = format!("TS{}", i);
            setup_card(&pool, &set, "1", i).await;
            setup_card_quantity(&pool, &set, "1", "user1", 1, 100).await;
            setup_price(&pool, i, i * 100).await;
        }
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            page: 0,
            page_size: 2,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 2);
        assert_eq!(result.total, 5);
        assert_eq!(result.page_size, 2);
    }

    #[sqlx::test]
    async fn get_paginated_returns_correct_page(pool: PgPool) {
        for i in 1..=4i32 {
            let set = format!("TS{}", i);
            setup_card(&pool, &set, "1", i).await;
            setup_card_quantity(&pool, &set, "1", "user1", 1, 100).await;
            setup_price(&pool, i, i * 100).await;
        }
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query_page0 = CollectionQuery {
            page: 0,
            page_size: 2,
            ..CollectionQuery::default()
        };
        let query_page1 = CollectionQuery {
            page: 1,
            page_size: 2,
            ..CollectionQuery::default()
        };

        let page0 = adapter.get_paginated("user1", query_page0).await.unwrap();
        let page1 = adapter.get_paginated("user1", query_page1).await.unwrap();

        assert_eq!(page0.items.len(), 2);
        assert_eq!(page1.items.len(), 2);
        assert_eq!(page0.page, 0);
        assert_eq!(page1.page, 1);

        let ids_page0: Vec<_> = page0
            .items
            .iter()
            .map(|c| c.id.set_code.to_string())
            .collect();
        let ids_page1: Vec<_> = page1
            .items
            .iter()
            .map(|c| c.id.set_code.to_string())
            .collect();
        assert!(ids_page0.iter().all(|id| !ids_page1.contains(id)));
    }

    #[sqlx::test]
    async fn get_paginated_sorts_by_avg_descending_by_default(pool: PgPool) {
        setup_card(&pool, "TS1", "1", 1).await;
        setup_card(&pool, "TS2", "1", 2).await;
        setup_card_quantity(&pool, "TS1", "1", "user1", 1, 100).await;
        setup_card_quantity(&pool, "TS2", "1", "user1", 1, 100).await;
        setup_price(&pool, 1, 300).await;
        setup_price(&pool, 2, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 2);
        let first_avg = result.items[0]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        let second_avg = result.items[1]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        assert!(first_avg >= second_avg);
    }

    #[sqlx::test]
    async fn get_paginated_sorts_by_set_code_ascending(pool: PgPool) {
        setup_card(&pool, "ZZZ", "1", 1).await;
        setup_card(&pool, "AAA", "1", 2).await;
        setup_card_quantity(&pool, "ZZZ", "1", "user1", 1, 100).await;
        setup_card_quantity(&pool, "AAA", "1", "user1", 1, 100).await;
        setup_price(&pool, 1, 100).await;
        setup_price(&pool, 2, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            sort_by: CollectionSortField::SetCode,
            sort_dir: SortDirection::Asc,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items[0].id.set_code.to_string(), "AAA");
        assert_eq!(result.items[1].id.set_code.to_string(), "ZZZ");
    }

    #[sqlx::test]
    async fn get_paginated_returns_cards_without_price_when_no_cardmarket_data(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 1, 200).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
    }

    #[sqlx::test]
    async fn get_paginated_returns_correct_metadata(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 3, 750).await;
        setup_price(&pool, 1, 400).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            page: 0,
            page_size: 10,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.page, 0);
        assert_eq!(result.page_size, 10);
        assert_eq!(result.total, 1);
    }

    #[sqlx::test]
    async fn get_paginated_sorts_by_avg_ascending(pool: PgPool) {
        setup_card(&pool, "TS1", "1", 1).await;
        setup_card(&pool, "TS2", "1", 2).await;
        setup_card_quantity(&pool, "TS1", "1", "user1", 1, 100).await;
        setup_card_quantity(&pool, "TS2", "1", "user1", 1, 100).await;
        setup_price(&pool, 1, 100).await;
        setup_price(&pool, 2, 300).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            sort_by: CollectionSortField::Avg,
            sort_dir: SortDirection::Asc,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 2);
        let first_avg = result.items[0]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        let second_avg = result.items[1]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        assert!(first_avg <= second_avg);
    }

    #[sqlx::test]
    async fn get_paginated_sorts_by_set_code_descending(pool: PgPool) {
        setup_card(&pool, "AAA", "1", 1).await;
        setup_card(&pool, "ZZZ", "1", 2).await;
        setup_card_quantity(&pool, "AAA", "1", "user1", 1, 100).await;
        setup_card_quantity(&pool, "ZZZ", "1", "user1", 1, 100).await;
        setup_price(&pool, 1, 100).await;
        setup_price(&pool, 2, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            sort_by: CollectionSortField::SetCode,
            sort_dir: SortDirection::Desc,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items[0].id.set_code.to_string(), "ZZZ");
        assert_eq!(result.items[1].id.set_code.to_string(), "AAA");
    }

    #[sqlx::test]
    async fn get_paginated_sorts_by_language_code_ascending(pool: PgPool) {
        sqlx::query!(
            r#"INSERT INTO set_name (set_code, name) VALUES ('TST', 'Test Set') ON CONFLICT DO NOTHING"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
               VALUES ('TST', '1', 'FR', false, 'Test Card', 'C', $1, 1),
                      ('TST', '2', 'EN', false, 'Test Card', 'C', $2, 2)
               ON CONFLICT DO NOTHING"#,
            Uuid::new_v4(),
            Uuid::new_v4(),
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
               VALUES ('TST', '1', 'FR', false, 'user1', 1, 100),
                      ('TST', '2', 'EN', false, 'user1', 1, 100)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        setup_price(&pool, 1, 100).await;
        setup_price(&pool, 2, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            sort_by: CollectionSortField::LanguageCode,
            sort_dir: SortDirection::Asc,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 2);
        let first_lang = result.items[0].id.language_code.to_string();
        let second_lang = result.items[1].id.language_code.to_string();
        assert!(first_lang <= second_lang);
    }

    #[sqlx::test]
    async fn get_paginated_returns_empty_page_when_offset_exceeds_total(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 1, 100).await;
        setup_price(&pool, 1, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            page: 5,
            page_size: 10,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert!(result.items.is_empty());
        assert_eq!(result.total, 1);
        assert_eq!(result.page, 5);
    }

    #[sqlx::test]
    async fn get_paginated_isolates_cards_between_multiple_users(pool: PgPool) {
        setup_card(&pool, "TS1", "1", 1).await;
        setup_card(&pool, "TS2", "1", 2).await;
        setup_card_quantity(&pool, "TS1", "1", "userA", 1, 100).await;
        setup_card_quantity(&pool, "TS2", "1", "userB", 1, 100).await;
        setup_price(&pool, 1, 100).await;
        setup_price(&pool, 2, 200).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);

        let result_a = adapter
            .get_paginated("userA", CollectionQuery::default())
            .await
            .unwrap();
        let result_b = adapter
            .get_paginated("userB", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result_a.total, 1);
        assert_eq!(result_b.total, 1);
        assert_eq!(result_a.items[0].id.set_code.to_string(), "TS1");
        assert_eq!(result_b.items[0].id.set_code.to_string(), "TS2");
    }

    #[sqlx::test]
    async fn get_paginated_uses_latest_price_when_multiple_dates_exist(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 1, 100).await;

        sqlx::query!(
            r#"INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30)
               VALUES (1, '2024-01-01', 10, 100, 10, 10, 10, 10),
                      (1, '2025-06-01', 20, 999, 20, 20, 20, 20)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        let avg = result.items[0]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        assert_eq!(avg, Some(999));
    }

    #[sqlx::test]
    async fn get_paginated_returns_foil_prices_for_foil_cards(pool: PgPool) {
        sqlx::query!(
            r#"INSERT INTO set_name (set_code, name) VALUES ('TST', 'Test Set') ON CONFLICT DO NOTHING"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
               VALUES ('TST', '1', 'EN', true, 'Foil Card', 'C', $1, 1)
               ON CONFLICT DO NOTHING"#,
            Uuid::new_v4(),
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
               VALUES ('TST', '1', 'EN', true, 'user1', 1, 100)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30, low_foil, avg_foil, trend_foil, avg1_foil, avg7_foil, avg30_foil)
               VALUES (1, CURRENT_DATE, 10, 50, 10, 10, 10, 10, 20, 777, 20, 20, 20, 20)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert!(result.items[0].id.foil);
        let avg = result.items[0]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        assert_eq!(avg, Some(777));
    }

    #[sqlx::test]
    async fn get_paginated_returns_correct_quantity_and_purchase_price(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 7, 1234).await;
        setup_price(&pool, 1, 500).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].quantity, 7);
        assert_eq!(result.items[0].purchase_price, 1234);
    }

    #[sqlx::test]
    async fn get_paginated_price_guide_is_none_when_no_cardmarket_data(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        setup_card_quantity(&pool, "TST", "1", "user1", 1, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert!(result.items[0].price_guide.is_none());
    }

    #[sqlx::test]
    async fn get_paginated_non_foil_card_does_not_use_foil_prices(pool: PgPool) {
        sqlx::query!(
            r#"INSERT INTO set_name (set_code, name) VALUES ('TST', 'Test Set') ON CONFLICT DO NOTHING"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
               VALUES ('TST', '1', 'EN', false, 'Normal Card', 'C', $1, 1)
               ON CONFLICT DO NOTHING"#,
            Uuid::new_v4(),
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
               VALUES ('TST', '1', 'EN', false, 'user1', 1, 100)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO cardmarket_price (id_produit, date, low, avg, trend, avg1, avg7, avg30, low_foil, avg_foil, trend_foil, avg1_foil, avg7_foil, avg30_foil)
               VALUES (1, CURRENT_DATE, 10, 123, 10, 10, 10, 10, 20, 999, 20, 20, 20, 20)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert!(!result.items[0].id.foil);
        let avg = result.items[0]
            .price_guide
            .as_ref()
            .and_then(|p| p.avg.value);
        assert_eq!(avg, Some(123));
    }

    #[sqlx::test]
    async fn get_paginated_card_not_in_collection_is_not_returned(pool: PgPool) {
        setup_card(&pool, "TST", "1", 1).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert!(result.items.is_empty());
        assert_eq!(result.total, 0);
    }

    #[sqlx::test]
    async fn get_paginated_sorts_by_language_code_descending(pool: PgPool) {
        sqlx::query!(
            r#"INSERT INTO set_name (set_code, name) VALUES ('TST', 'Test Set') ON CONFLICT DO NOTHING"#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id, cardmarket_id)
               VALUES ('TST', '1', 'FR', false, 'Test Card', 'C', $1, 1),
                      ('TST', '2', 'EN', false, 'Test Card', 'C', $2, 2)
               ON CONFLICT DO NOTHING"#,
            Uuid::new_v4(),
            Uuid::new_v4(),
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
               VALUES ('TST', '1', 'FR', false, 'user1', 1, 100),
                      ('TST', '2', 'EN', false, 'user1', 1, 100)"#
        )
        .execute(&pool)
        .await
        .unwrap();

        setup_price(&pool, 1, 100).await;
        setup_price(&pool, 2, 100).await;
        refresh_view(&pool).await;

        let adapter = CollectionRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            sort_by: CollectionSortField::LanguageCode,
            sort_dir: SortDirection::Desc,
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 2);
        let first_lang = result.items[0].id.language_code.to_string();
        let second_lang = result.items[1].id.language_code.to_string();
        assert!(first_lang >= second_lang);
    }
}
