use crate::application::error::AppError;
use crate::application::repository::CardPricesViewRepository;
use crate::domain::card::Card;
use crate::domain::collection::{CollectionQuery, PaginatedCollection};
use crate::infrastructure::adapter_out::repository::entities::CardWithPriceEntity;
use async_trait::async_trait;
use sqlx::{AssertSqlSafe, Pool, Postgres, query_as, query_scalar};

/// Builds the "AND ..." filter clause (search, rarity, sets, price range) for the
/// collection query, starting bind placeholders at `start_idx`.
/// Returns (filter_clause, order_prefix, next_idx).
fn build_filter_clause(query: &CollectionQuery, start_idx: u32) -> (String, String, u32) {
    let mut idx = start_idx;
    let mut conditions = Vec::new();
    let mut order_prefix = String::new();

    if query.search_query.is_some() {
        conditions.push(format!(
            "(cp.name ILIKE '%' || ${idx} || '%' OR ${idx} <% cp.name)"
        ));
        order_prefix = format!("word_similarity(${idx}, cp.name) DESC,");
        idx += 1;
    }
    if !query.rarity.is_empty() {
        conditions.push(format!("cp.rarity = ANY(${idx})"));
        idx += 1;
    }
    if !query.sets.is_empty() {
        conditions.push(format!("cp.set_code = ANY(${idx})"));
        idx += 1;
    }
    if query.price_min.is_some() {
        conditions.push(format!("cp.trend >= ${idx}"));
        idx += 1;
    }
    if query.price_max.is_some() {
        conditions.push(format!("cp.trend <= ${idx}"));
        idx += 1;
    }

    let filter_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("AND {}", conditions.join(" AND "))
    };

    (filter_clause, order_prefix, idx)
}

pub struct CardPricesViewRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardPricesViewRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardPricesViewRepository for CardPricesViewRepositoryAdapter {
    async fn refresh(&self) -> Result<(), AppError> {
        sqlx::query("REFRESH MATERIALIZED VIEW CONCURRENTLY mv_card_prices")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_paginated(
        &self,
        user_id: &str,
        query: CollectionQuery,
    ) -> Result<PaginatedCollection, AppError> {
        let (filter_clause, order_prefix, _) = build_filter_clause(&query, 4);
        let (count_filter_clause, _, _) = build_filter_clause(&query, 2);

        let sql = format!(
            r#"SELECT
                 cp.set_code,
                 sn.name AS set_name,
                 cp.collector_number,
                 cp.language_code,
                 cp.foil,
                 cp.name,
                 cp.rarity,
                 cp.scryfall_id,
                 cp.quantity,
                 cp.purchase_price,
                 cp.avg,
                 cp.low,
                 cp.trend
               FROM mv_card_prices cp
               JOIN set_name sn ON sn.set_code = cp.set_code
               WHERE cp.user_id = $1
               {}
               ORDER BY {} {} {} NULLS LAST, cp.name
               LIMIT $2 OFFSET $3"#,
            filter_clause, order_prefix, query.sort_by, query.sort_dir,
        );

        let offset = (query.page * query.page_size) as i64;
        let limit = query.page_size as i64;

        let mut base_query = query_as::<_, CardWithPriceEntity>(AssertSqlSafe(sql.as_str()))
            .bind(user_id)
            .bind(limit)
            .bind(offset);
        if let Some(q) = &query.search_query {
            base_query = base_query.bind(q.clone());
        }
        if !query.rarity.is_empty() {
            base_query = base_query.bind(
                query
                    .rarity
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>(),
            );
        }
        if !query.sets.is_empty() {
            base_query = base_query.bind(query.sets.clone());
        }
        if let Some(v) = query.price_min {
            base_query = base_query.bind(v as i64);
        }
        if let Some(v) = query.price_max {
            base_query = base_query.bind(v as i64);
        }

        let entities = base_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::RepositoryError(e.to_string()))?;

        let count_sql = format!(
            "SELECT COUNT(*) FROM mv_card_prices cp WHERE cp.user_id = $1 {}",
            count_filter_clause
        );

        let mut base_count =
            query_scalar::<_, i64>(AssertSqlSafe(count_sql.as_str())).bind(user_id);
        if let Some(q) = &query.search_query {
            base_count = base_count.bind(q.clone());
        }
        if !query.rarity.is_empty() {
            base_count = base_count.bind(
                query
                    .rarity
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>(),
            );
        }
        if !query.sets.is_empty() {
            base_count = base_count.bind(query.sets.clone());
        }
        if let Some(v) = query.price_min {
            base_count = base_count.bind(v as i64);
        }
        if let Some(v) = query.price_max {
            base_count = base_count.bind(v as i64);
        }

        let total: i64 = base_count
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
    use crate::domain::collection::{CollectionSortField, SortDirection};
    use crate::domain::rarity_code::RarityCode;
    use crate::infrastructure::adapter_out::repository::common_repository_tests::{
        insert_card, insert_collection_entry, insert_price, insert_set, refresh_view,
    };
    use crate::infrastructure::adapter_out::repository::entities::{
        CardMarketPriceEntity, PriceGuideEntity,
    };
    use chrono::{NaiveDate, Utc};
    use sqlx::{PgPool, Pool, Postgres};

    impl CardMarketPriceEntity {
        pub fn simple(id_produit: i32, avg: i32) -> Self {
            Self::simple_at(id_produit, chrono::Local::now().date_naive(), avg)
        }

        pub fn simple_at(id_produit: i32, date: NaiveDate, avg: i32) -> Self {
            Self {
                id_produit,
                date,
                normal: PriceGuideEntity {
                    low: Some(avg / 2),
                    avg: Some(avg),
                    trend: Some(avg),
                },
                foil: PriceGuideEntity::empty(),
            }
        }

        pub fn with_foil(id_produit: i32, avg: i32, avg_foil: i32) -> Self {
            Self::with_foil_at(id_produit, chrono::Local::now().date_naive(), avg, avg_foil)
        }

        pub fn with_foil_at(id_produit: i32, date: NaiveDate, avg: i32, avg_foil: i32) -> Self {
            Self {
                id_produit,
                date,
                normal: PriceGuideEntity {
                    low: Some(avg / 2),
                    avg: Some(avg),
                    trend: Some(avg),
                },
                foil: PriceGuideEntity {
                    low: Some(avg_foil / 2),
                    avg: Some(avg_foil),
                    trend: Some(avg_foil),
                },
            }
        }
    }

    #[sqlx::test]
    async fn test_refresh_materialized_view(pool: Pool<Postgres>) {
        let adapter = CardPricesViewRepositoryAdapter::new(pool);

        let result = adapter.refresh().await;

        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn get_paginated_returns_empty_when_no_cards_in_collection(pool: PgPool) {
        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert!(result.items.is_empty());
        assert_eq!(result.total, 0);
    }

    #[sqlx::test]
    async fn get_paginated_returns_cards_for_the_given_user(pool: PgPool) {
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 2, 500, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 200)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
    }

    #[sqlx::test]
    async fn get_paginated_respects_page_size(pool: PgPool) {
        for i in 1..=5i32 {
            let set = format!("TS{}", i);
            insert_set(&pool, &set).await;
            insert_card(&pool, &set, "1", "EN", false, "Test Card", i).await;
            insert_collection_entry(&pool, &set, "1", "EN", false, "user1", 1, 100, Utc::now())
                .await;
            insert_price(&pool, CardMarketPriceEntity::simple(i, i * 100)).await;
        }
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
            insert_set(&pool, &set).await;
            insert_card(&pool, &set, "1", "EN", false, "Test Card", i).await;
            insert_collection_entry(&pool, &set, "1", "EN", false, "user1", 1, 100, Utc::now())
                .await;
            insert_price(&pool, CardMarketPriceEntity::simple(i, i * 100)).await;
        }
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TS1").await;
        insert_card(&pool, "TS1", "1", "EN", false, "Test Card", 1).await;
        insert_set(&pool, "TS2").await;
        insert_card(&pool, "TS2", "1", "EN", false, "Test Card", 2).await;
        insert_collection_entry(&pool, "TS1", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TS2", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 300)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "ZZZ").await;
        insert_card(&pool, "ZZZ", "1", "EN", false, "Test Card", 1).await;
        insert_set(&pool, "AAA").await;
        insert_card(&pool, "AAA", "1", "EN", false, "Test Card", 2).await;
        insert_collection_entry(&pool, "ZZZ", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "AAA", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
    async fn get_paginated_sorts_by_language_code_ascending(pool: PgPool) {
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "FR", false, "Test Card", 1).await;
        insert_card(&pool, "TST", "2", "EN", false, "Test Card", 2).await;
        insert_collection_entry(&pool, "TST", "1", "FR", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TST", "2", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TS1").await;
        insert_card(&pool, "TS1", "1", "EN", false, "Test Card", 1).await;
        insert_set(&pool, "TS2").await;
        insert_card(&pool, "TS2", "1", "EN", false, "Test Card", 2).await;
        insert_collection_entry(&pool, "TS1", "1", "EN", false, "userA", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TS2", "1", "EN", false, "userB", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 200)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);

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
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;

        insert_price(
            &pool,
            CardMarketPriceEntity::simple_at(1, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), 100),
        )
        .await;
        insert_price(
            &pool,
            CardMarketPriceEntity::simple_at(1, NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), 999),
        )
        .await;

        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", true, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", true, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::with_foil(1, 50, 777)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 7, 1234, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 500)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
        assert!(result.items[0].price_guide.is_none());
    }

    #[sqlx::test]
    async fn get_paginated_non_foil_card_does_not_use_foil_prices(pool: PgPool) {
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::with_foil(1, 123, 999)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
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
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Test Card", 1).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let result = adapter
            .get_paginated("user1", CollectionQuery::default())
            .await
            .unwrap();

        assert!(result.items.is_empty());
        assert_eq!(result.total, 0);
    }

    #[sqlx::test]
    async fn get_paginated_filters_by_search_query_fuzzy(pool: PgPool) {
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Goblin Guide", 1).await;
        insert_card(&pool, "TST", "2", "EN", false, "Sol Ring", 2).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TST", "2", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            search_query: Some("gob".to_string()),
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
        assert_eq!(result.items[0].name, "Goblin Guide");
    }

    #[sqlx::test]
    async fn get_paginated_filters_by_rarity(pool: PgPool) {
        use crate::infrastructure::adapter_out::repository::common_repository_tests::insert_card_with_rarity;

        insert_set(&pool, "TST").await;
        insert_card_with_rarity(&pool, "TST", "1", "EN", false, "Common Card", 1, "C").await;
        insert_card_with_rarity(&pool, "TST", "2", "EN", false, "Mythic Card", 2, "M").await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TST", "2", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            rarity: vec![RarityCode::M],
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
        assert_eq!(result.items[0].name, "Mythic Card");
    }

    #[sqlx::test]
    async fn get_paginated_filters_by_sets(pool: PgPool) {
        insert_set(&pool, "TS1").await;
        insert_set(&pool, "TS2").await;
        insert_card(&pool, "TS1", "1", "EN", false, "Card A", 1).await;
        insert_card(&pool, "TS2", "1", "EN", false, "Card B", 2).await;
        insert_collection_entry(&pool, "TS1", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TS2", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 100)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            sets: vec!["TS2".to_string()],
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
        assert_eq!(result.items[0].name, "Card B");
    }

    #[sqlx::test]
    async fn get_paginated_filters_by_price_range(pool: PgPool) {
        insert_set(&pool, "TST").await;
        insert_card(&pool, "TST", "1", "EN", false, "Cheap Card", 1).await;
        insert_card(&pool, "TST", "2", "EN", false, "Expensive Card", 2).await;
        insert_collection_entry(&pool, "TST", "1", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_collection_entry(&pool, "TST", "2", "EN", false, "user1", 1, 100, Utc::now()).await;
        insert_price(&pool, CardMarketPriceEntity::simple(1, 100)).await;
        insert_price(&pool, CardMarketPriceEntity::simple(2, 5000)).await;
        refresh_view(&pool).await;

        let adapter = CardPricesViewRepositoryAdapter::new(pool);
        let query = CollectionQuery {
            price_min: Some(1000),
            price_max: Some(10000),
            ..CollectionQuery::default()
        };
        let result = adapter.get_paginated("user1", query).await.unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.total, 1);
        assert_eq!(result.items[0].name, "Expensive Card");
    }
}
