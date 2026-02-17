use crate::application::caller::ScryfallCaller;
use crate::application::error::AppError;
use crate::application::repository::CardRepository;
use crate::application::use_case::UpdateCardMarketIdUseCase;
use crate::domain::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateCardMarketIdService {
    card_repository: Arc<dyn CardRepository>,
    scryfall_caller: Arc<dyn ScryfallCaller>,
}

impl UpdateCardMarketIdService {
    pub fn new(
        card_repository: Arc<dyn CardRepository>,
        scryfall_caller: Arc<dyn ScryfallCaller>,
    ) -> Self {
        Self {
            card_repository,
            scryfall_caller,
        }
    }
}

#[async_trait]
impl UpdateCardMarketIdUseCase for UpdateCardMarketIdService {
    async fn update_cards(&self) -> Result<(), AppError> {
        let cards = self.card_repository.get_all_without_cardmarket_id().await?;

        for c in cards {
            let cardmarket_id = self.scryfall_caller.get_card_market_id(c.scryfall_id).await;
            if let Ok(id) = cardmarket_id {
                let updated_card = c.with_cardmarket_id(id);
                if let Err(e) = self.card_repository.save(User::new(), updated_card).await {
                    eprintln!("Failed to update card with CardMarket ID: {:?}", e);
                }
                println!("Updated card {} with CardMarket ID: {:?}", c.name, id);
            } else if let Err(e) = cardmarket_id {
                eprintln!("Failed to fetch CardMarket ID for card {}: {:?}", c.name, e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::caller::MockScryfallCaller;
    use crate::application::repository::MockCardRepository;
    use crate::domain::card::Card;
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::SetCode;

    #[tokio::test]
    async fn updates_all_cards_with_valid_cardmarket_ids() {
        let mut card_repository = MockCardRepository::new();
        let mut scryfall_caller = MockScryfallCaller::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move {
                    Ok(vec![
                        Card::new(
                            SetCode::new("FDN"),
                            "Foundations",
                            "0",
                            LanguageCode::FR,
                            false,
                            "Card One",
                            1,
                            1,
                        ),
                        Card::new(
                            SetCode::new("FDN"),
                            "Foundations",
                            "1",
                            LanguageCode::FR,
                            false,
                            "Card Two",
                            1,
                            1,
                        ),
                    ])
                })
            });

        scryfall_caller
            .expect_get_card_market_id()
            .returning(|_| Box::pin(async move { Ok(Some(123)) }));

        card_repository
            .expect_save()
            .returning(|_, _| Box::pin(async move { Ok(()) }));

        let service =
            UpdateCardMarketIdService::new(Arc::new(card_repository), Arc::new(scryfall_caller));

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn continues_processing_when_fetching_cardmarket_id_fails() {
        let mut card_repository = MockCardRepository::new();
        let mut scryfall_caller = MockScryfallCaller::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move {
                    Ok(vec![Card::new(
                        SetCode::new("FDN"),
                        "Foundations",
                        "0",
                        LanguageCode::FR,
                        false,
                        "Card One",
                        1,
                        1,
                    )])
                })
            });

        scryfall_caller.expect_get_card_market_id().returning(|_| {
            Box::pin(async move { Err(AppError::CallError("Scryfall API error".to_string())) })
        });

        let service =
            UpdateCardMarketIdService::new(Arc::new(card_repository), Arc::new(scryfall_caller));

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn continues_processing_when_saving_card_fails() {
        let mut card_repository = MockCardRepository::new();
        let mut scryfall_caller = MockScryfallCaller::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move {
                    Ok(vec![Card::new(
                        SetCode::new("FDN"),
                        "Foundations",
                        "0",
                        LanguageCode::FR,
                        false,
                        "Card One",
                        1,
                        1,
                    )])
                })
            });

        scryfall_caller
            .expect_get_card_market_id()
            .returning(|_| Box::pin(async move { Ok(Some(123)) }));

        card_repository.expect_save().returning(|_, _| {
            Box::pin(async move { Err(AppError::RepositoryError("Save failed".to_string())) })
        });

        let service =
            UpdateCardMarketIdService::new(Arc::new(card_repository), Arc::new(scryfall_caller));

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn handles_empty_card_list() {
        let mut card_repository = MockCardRepository::new();
        let scryfall_caller = MockScryfallCaller::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || Box::pin(async move { Ok(vec![]) }));

        let service =
            UpdateCardMarketIdService::new(Arc::new(card_repository), Arc::new(scryfall_caller));

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn propagates_error_from_card_repository_get_all() {
        let mut card_repository = MockCardRepository::new();
        let scryfall_caller = MockScryfallCaller::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move { Err(AppError::RepositoryError("DB error".to_string())) })
            });

        let service =
            UpdateCardMarketIdService::new(Arc::new(card_repository), Arc::new(scryfall_caller));

        let result = service.update_cards().await;
        assert!(matches!(
            result,
            Err(AppError::RepositoryError(s)) if s == "DB error"
        ));
    }
}
