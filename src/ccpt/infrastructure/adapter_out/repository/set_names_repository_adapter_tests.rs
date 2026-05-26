use super::*;
use sqlx::PgPool;

#[sqlx::test]
async fn test_no_card_exists(pool: PgPool) {
    let exists = SetNameRepositoryAdapter::new(pool)
        .exists_by_code(SetCode::new("ECC"))
        .await
        .unwrap();
    assert!(!exists, "no set should exist in the database");
}

#[sqlx::test]
async fn exists_by_code_returns_true_for_existing_set_code(pool: PgPool) {
    let adapter = SetNameRepositoryAdapter::new(pool.clone());
    let exists = adapter.exists_by_code(SetCode::new("ECL")).await.unwrap();
    assert!(exists, "set should exist in the database");
}

#[sqlx::test]
async fn save_does_not_insert_duplicate_set_code(pool: PgPool) {
    let adapter = SetNameRepositoryAdapter::new(pool.clone());

    let set_name = SetName {
        code: SetCode::new("ECL"),
        name: "Lorwyn Eclipsed 2".to_string(),
    };

    adapter.save(set_name).await.unwrap();

    let result = sqlx::query!("SELECT name FROM set_name WHERE set_code = $1", "ECL")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(
        result.name, "Lorwyn Eclipsed 2",
        "existing set name should be overridden"
    );
}

#[sqlx::test]
async fn save_inserts_new_set_name(pool: PgPool) {
    let adapter = SetNameRepositoryAdapter::new(pool.clone());

    let set_name = SetName {
        code: SetCode::new("ECC"),
        name: "Lorwyn Eclipsed Commander".to_string(),
    };

    adapter.save(set_name).await.unwrap();

    let result = sqlx::query!("SELECT name FROM set_name WHERE set_code = $1", "ECC")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(
        result.name, "Lorwyn Eclipsed Commander",
        "new set should be inserted into the database"
    );
}
