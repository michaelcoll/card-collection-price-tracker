use super::*;
use sqlx::{Pool, Postgres};

#[sqlx::test]
async fn test_refresh_materialized_view(pool: Pool<Postgres>) {
    let adapter = CardPricesViewRepositoryAdapter::new(pool);

    let result = adapter.refresh().await;

    assert!(result.is_ok());
}
