use super::*;
use crate::domain::language_code::LanguageCode;
use crate::domain::rarity_code::RarityCode;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_no_card_exists(pool: PgPool) {
    let vec = CardRepositoryAdapter::new(pool)
        .get_all(User::for_testing())
        .await
        .unwrap();
    assert!(vec.is_empty(), "no cards should exist in the database");
}

#[sqlx::test]
async fn test_get_user_id(pool: PgPool) {
    let repository = CardRepositoryAdapter::new(pool);

    let card = Card::new(
        "FDN",
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        500,
    );

    repository
        .save(User::for_testing(), card.clone())
        .await
        .unwrap();

    let cards = repository.get_all(User::for_testing()).await.unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0], card);
}

#[sqlx::test]
async fn save_card_updates_existing_card(pool: PgPool) {
    let repository = CardRepositoryAdapter::new(pool);

    let card = Card::new(
        "FDN",
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        500,
    );
    repository
        .save(User::for_testing(), card.clone())
        .await
        .unwrap();

    let updated_card = Card::new(
        "FDN",
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        5,
        1500,
    );
    repository
        .save(User::for_testing(), updated_card.clone())
        .await
        .unwrap();

    let cards = repository.get_all(User::for_testing()).await.unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0], updated_card);
}

#[sqlx::test]
async fn delete_all_removes_all_cards(pool: PgPool) {
    let repository = CardRepositoryAdapter::new(pool);

    let card1 = Card::new(
        "FDN",
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        500,
    );
    let card2 = Card::new(
        "FDN",
        "Foundations",
        "12",
        LanguageCode::EN,
        true,
        "Goblin Boarders",
        RarityCode::C,
        2,
        1000,
    );

    repository.save(User::for_testing(), card1).await.unwrap();
    repository.save(User::for_testing(), card2).await.unwrap();

    repository.delete_all(User::for_testing()).await.unwrap();

    let cards = repository.get_all(User::for_testing()).await.unwrap();
    assert!(
        cards.is_empty(),
        "all cards should be deleted from the database"
    );
}

#[sqlx::test]
async fn get_all_returns_multiple_cards(pool: PgPool) {
    let repository = CardRepositoryAdapter::new(pool);

    let card1 = Card::new(
        "FDN",
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        500,
    );
    let card2 = Card::new(
        "FDN",
        "Foundations",
        "12",
        LanguageCode::EN,
        true,
        "Goblin Boarders",
        RarityCode::C,
        2,
        1000,
    );

    repository
        .save(User::for_testing(), card1.clone())
        .await
        .unwrap();
    repository
        .save(User::for_testing(), card2.clone())
        .await
        .unwrap();

    let cards = repository.get_all(User::for_testing()).await.unwrap();
    assert_eq!(cards.len(), 2);
    assert!(cards.contains(&card1));
    assert!(cards.contains(&card2));
}

#[sqlx::test]
async fn get_all_without_cardmarket_id_returns_only_cards_without_cardmarket_id(pool: PgPool) {
    let repository = CardRepositoryAdapter::new(pool);

    let card_without_id = Card::new(
        "FDN",
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        500,
    );
    let card_with_id = Card::new_full(
        "FDN",
        "Foundations",
        "12",
        LanguageCode::EN,
        true,
        "Goblin Boarders",
        RarityCode::C,
        2,
        1000,
        Uuid::default(),
        Some(123),
        None,
    );

    repository
        .save(User::for_testing(), card_without_id.clone())
        .await
        .unwrap();
    repository
        .save(User::for_testing(), card_with_id.clone())
        .await
        .unwrap();
    repository
        .update_cardmarket_id(card_with_id.id, card_with_id.cardmarket_id)
        .await
        .unwrap();

    let cards = repository.get_all_without_cardmarket_id().await.unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0], (card_without_id.id, card_without_id.scryfall_id));
}
