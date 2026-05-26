use super::*;
use crate::application::repository::{
    MockCardPricesViewRepository, MockCardRepository, MockSetNameRepository,
};
use crate::application::use_case::MockEnqueueCardMarketIdUpdateUseCase;
use crate::domain::card::Card;
use crate::domain::language_code::LanguageCode;
use crate::domain::rarity_code::RarityCode;
use crate::domain::set_name::{SetCode, SetName};
use chrono::{DateTime, Utc};
use mockall::predicate::eq;
use uuid::Uuid;

#[tokio::test]
async fn import_cards_saves_cards_and_set_names_successfully() {
    let mut card_repository = MockCardRepository::new();
    let mut set_name_repository = MockSetNameRepository::new();
    let mut enqueue_use_case = MockEnqueueCardMarketIdUpdateUseCase::new();

    let set_code = SetCode::new("FDN");
    let set_name = SetName {
        code: set_code.clone(),
        name: "Foundations".to_string(),
    };
    let card = Card::new_full(
        set_code.clone(),
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        8,
        Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
        None,
        Some(
            DateTime::parse_from_rfc3339("2026-02-05T20:44:45.815Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
    );

    card_repository
        .expect_delete_all()
        .with(eq(User::for_testing()))
        .returning(|_| Box::pin(async { Ok(()) }));
    set_name_repository
        .expect_exists_by_code()
        .with(eq(set_code.clone()))
        .returning(|_| Box::pin(async { Ok(false) }));
    set_name_repository
        .expect_save()
        .with(eq(set_name.clone()))
        .returning(|_| Box::pin(async { Ok(()) }));
    card_repository
        .expect_save()
        .with(eq(User::for_testing()), eq(card.clone()))
        .returning(|_, _| Box::pin(async { Ok(()) }));
    enqueue_use_case
        .expect_enqueue_pending_updates()
        .returning(|| Box::pin(async { Ok(2) }));

    let mut card_prices_view_repository = MockCardPricesViewRepository::new();
    card_prices_view_repository
        .expect_refresh()
        .returning(|| Box::pin(async { Ok(()) }));

    let service = ImportCardService::new(
        Arc::new(card_repository),
        Arc::new(set_name_repository),
        Arc::new(enqueue_use_case),
        Arc::new(card_prices_view_repository),
    );

    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";
    let result = service.import_cards(csv, User::for_testing()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn import_cards_rollback_on_card_save_error() {
    let mut card_repository = MockCardRepository::new();
    let mut set_name_repository = MockSetNameRepository::new();

    let set_code = SetCode::new("FDN");
    let set_name = SetName {
        code: set_code.clone(),
        name: "Foundations".to_string(),
    };
    let card = Card::new_full(
        set_code.clone(),
        "Foundations",
        "0",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        0,
        0,
        Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
        None,
        Some(
            DateTime::parse_from_rfc3339("2026-02-05T20:44:45.815Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
    );

    card_repository
        .expect_delete_all()
        .with(eq(User::for_testing()))
        .returning(|_| Box::pin(async { Ok(()) }));
    set_name_repository
        .expect_exists_by_code()
        .with(eq(set_name.code.clone()))
        .returning(|_| Box::pin(async { Ok(false) }));
    set_name_repository
        .expect_save()
        .with(eq(set_name.clone()))
        .returning(|_| Box::pin(async { Ok(()) }));
    card_repository
        .expect_save()
        .with(eq(User::for_testing()), eq(card.clone()))
        .returning(|_, _| {
            Box::pin(async { Err(AppError::RepositoryError("Save failed".to_string())) })
        });

    let mock_enqueue = MockEnqueueCardMarketIdUpdateUseCase::new();
    let card_prices_view_repository = MockCardPricesViewRepository::new();

    let service = ImportCardService::new(
        Arc::new(card_repository),
        Arc::new(set_name_repository),
        Arc::new(mock_enqueue),
        Arc::new(card_prices_view_repository),
    );

    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,0,normal,common,0,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.00,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";
    let result = service.import_cards(csv, User::for_testing()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn import_cards_does_not_save_duplicate_set_name() {
    let mut card_repository = MockCardRepository::new();
    let mut set_name_repository = MockSetNameRepository::new();
    let mut enqueue_use_case = MockEnqueueCardMarketIdUpdateUseCase::new();

    let set_code = SetCode::new("FDN");
    let card = Card::new_full(
        set_code.clone(),
        "Foundations",
        "87",
        LanguageCode::FR,
        false,
        "Goblin Boarders",
        RarityCode::C,
        3,
        8,
        Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
        None,
        Some(
            DateTime::parse_from_rfc3339("2026-02-05T20:44:45.815Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
    );

    card_repository
        .expect_delete_all()
        .returning(|_| Box::pin(async { Ok(()) }));
    set_name_repository
        .expect_exists_by_code()
        .with(eq(set_code.clone()))
        .returning(|_| Box::pin(async { Ok(true) }));
    card_repository
        .expect_save()
        .with(eq(User::for_testing()), eq(card.clone()))
        .returning(|_, _| Box::pin(async { Ok(()) }));
    enqueue_use_case
        .expect_enqueue_pending_updates()
        .returning(|| Box::pin(async { Ok(1) }));

    let mut card_prices_view_repository = MockCardPricesViewRepository::new();
    card_prices_view_repository
        .expect_refresh()
        .returning(|| Box::pin(async { Ok(()) }));

    let service = ImportCardService::new(
        Arc::new(card_repository),
        Arc::new(set_name_repository),
        Arc::new(enqueue_use_case),
        Arc::new(card_prices_view_repository),
    );

    let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency,Added\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR,2026-02-05T20:44:45.815Z";
    let result = service.import_cards(csv, User::for_testing()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn import_cards_fails_on_parse_error() {
    let card_repository = MockCardRepository::new();
    let set_name_repository = MockSetNameRepository::new();
    let mock_enqueue = MockEnqueueCardMarketIdUpdateUseCase::new();
    let card_prices_view_repository = MockCardPricesViewRepository::new();

    let service = ImportCardService::new(
        Arc::new(card_repository),
        Arc::new(set_name_repository),
        Arc::new(mock_enqueue),
        Arc::new(card_prices_view_repository),
    );

    let invalid_csv = "Invalid,Data";
    let result = service.import_cards(invalid_csv, User::for_testing()).await;

    assert!(result.is_err());
}
