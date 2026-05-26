use crate::application::error::AppError;
use crate::application::repository::CollectionRepository;
use crate::domain::card::Card;
use crate::domain::collection::{CollectionQuery, PaginatedCollection};
use crate::infrastructure::adapter_out::repository::entities::CardWithPriceEntity;
use async_trait::async_trait;
use sqlx::AssertSqlSafe;
use sqlx::{Pool, Postgres, query_as, query_scalar};

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
        let (where_clause, order_prefix) = match &query.search_query {
            Some(_) => (
                "AND (cp.name ILIKE '%' || $4 || '%' OR $4 <% cp.name)".to_string(),
                "word_similarity($4, cp.name) DESC,".to_string(),
            ),
            None => (String::new(), String::new()),
        };

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
                 cp.trend,
                 cp.avg1,
                 cp.avg7,
                 cp.avg30
               FROM mv_card_prices cp
               JOIN set_name sn ON sn.set_code = cp.set_code
               WHERE cp.user_id = $1
               {}
               ORDER BY {} {} {} NULLS LAST, cp.name
               LIMIT $2 OFFSET $3"#,
            where_clause, order_prefix, query.sort_by, query.sort_dir,
        );

        let offset = (query.page * query.page_size) as i64;
        let limit = query.page_size as i64;

        let base_query = query_as::<_, CardWithPriceEntity>(AssertSqlSafe(sql.as_str()))
            .bind(user_id)
            .bind(limit)
            .bind(offset);

        let entities = match &query.search_query {
            Some(q) => base_query.bind(q.clone()),
            None => base_query,
        }
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::RepositoryError(e.to_string()))?;

        let count_sql = match &query.search_query {
            Some(_) => "SELECT COUNT(*) FROM mv_card_prices cp WHERE cp.user_id = $1 AND (cp.name ILIKE '%' || $2 || '%' OR $2 <% cp.name)".to_string(),
            None => "SELECT COUNT(*) FROM mv_card_prices cp WHERE cp.user_id = $1".to_string(),
        };

        let base_count = query_scalar::<_, i64>(AssertSqlSafe(count_sql.as_str())).bind(user_id);
        let total: i64 = match &query.search_query {
            Some(q) => base_count.bind(q.clone()),
            None => base_count,
        }
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
#[path = "collection_repository_adapter_tests.rs"]
mod tests;
