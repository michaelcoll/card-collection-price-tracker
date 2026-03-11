use crate::application::caller::ScryfallCaller;
use crate::application::error::AppError;
use crate::application::repository::CardRepository;
use crate::application::use_case::{
    CardCollectionPriceCalculationUseCase, UpdateCardMarketIdUseCase,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateCardMarketIdService {
    card_repository: Arc<dyn CardRepository>,
    scryfall_caller: Arc<dyn ScryfallCaller>,
    price_calculation: Arc<dyn CardCollectionPriceCalculationUseCase>,
}

impl UpdateCardMarketIdService {
    pub fn new(
        card_repository: Arc<dyn CardRepository>,
        scryfall_caller: Arc<dyn ScryfallCaller>,
        price_calculation: Arc<dyn CardCollectionPriceCalculationUseCase>,
    ) -> Self {
        Self {
            card_repository,
            scryfall_caller,
            price_calculation,
        }
    }
}

#[async_trait]
impl UpdateCardMarketIdUseCase for UpdateCardMarketIdService {
    async fn update_cards(&self) -> Result<(), AppError> {
        println!("Updating cards...");

        let cards = self.card_repository.get_all_without_cardmarket_id().await?;

        for (card_id, scryfall_id) in cards {
            let cardmarket_id = self.scryfall_caller.get_card_market_id(scryfall_id).await;
            if let Ok(id) = cardmarket_id {
                if let Err(e) = self
                    .card_repository
                    .update_cardmarket_id(card_id.clone(), id)
                    .await
                {
                    eprintln!("Failed to update card with CardMarket ID: {:?}", e);
                }
                println!("Updated card {} with CardMarket ID: {:?}", card_id, id);
            } else if let Err(e) = cardmarket_id {
                eprintln!(
                    "Failed to fetch CardMarket ID for card {}: {:?}",
                    card_id, e
                );
            }
        }

        self.price_calculation.calculate_total_price().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::caller::MockScryfallCaller;
    use crate::application::repository::MockCardRepository;
    use crate::application::use_case::MockCardCollectionPriceCalculationUseCase;
    use crate::domain::card::CardId;
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::SetCode;
    use uuid::Uuid;

    #[tokio::test]
    async fn updates_all_cards_with_valid_cardmarket_ids() {
        let mut card_repository = MockCardRepository::new();
        let mut scryfall_caller = MockScryfallCaller::new();
        let mut mock_card_collection_price_calculation_use_case =
            MockCardCollectionPriceCalculationUseCase::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move {
                    Ok(vec![
                        (
                            CardId::new(SetCode::new("FDN"), "0", LanguageCode::FR, false),
                            Uuid::default(),
                        ),
                        (
                            CardId::new(SetCode::new("FDN"), "1", LanguageCode::FR, false),
                            Uuid::default(),
                        ),
                    ])
                })
            });

        scryfall_caller
            .expect_get_card_market_id()
            .returning(|_| Box::pin(async move { Ok(Some(123)) }));

        card_repository
            .expect_update_cardmarket_id()
            .returning(|_, _| Box::pin(async move { Ok(()) }));
        mock_card_collection_price_calculation_use_case
            .expect_calculate_total_price()
            .returning(|| Box::pin(async move { Ok(()) }));

        let service = UpdateCardMarketIdService::new(
            Arc::new(card_repository),
            Arc::new(scryfall_caller),
            Arc::new(mock_card_collection_price_calculation_use_case),
        );

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn continues_processing_when_fetching_cardmarket_id_fails() {
        let mut card_repository = MockCardRepository::new();
        let mut scryfall_caller = MockScryfallCaller::new();
        let mut mock_card_collection_price_calculation_use_case =
            MockCardCollectionPriceCalculationUseCase::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move {
                    Ok(vec![(
                        CardId::new(SetCode::new("FDN"), "0", LanguageCode::FR, false),
                        Uuid::default(),
                    )])
                })
            });

        scryfall_caller.expect_get_card_market_id().returning(|_| {
            Box::pin(async move { Err(AppError::CallError("Scryfall API error".to_string())) })
        });
        mock_card_collection_price_calculation_use_case
            .expect_calculate_total_price()
            .returning(|| Box::pin(async move { Ok(()) }));

        let service = UpdateCardMarketIdService::new(
            Arc::new(card_repository),
            Arc::new(scryfall_caller),
            Arc::new(mock_card_collection_price_calculation_use_case),
        );

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn continues_processing_when_saving_card_fails() {
        let mut card_repository = MockCardRepository::new();
        let mut scryfall_caller = MockScryfallCaller::new();
        let mut mock_card_collection_price_calculation_use_case =
            MockCardCollectionPriceCalculationUseCase::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move {
                    Ok(vec![(
                        CardId::new(SetCode::new("FDN"), "0", LanguageCode::FR, false),
                        Uuid::default(),
                    )])
                })
            });

        scryfall_caller
            .expect_get_card_market_id()
            .returning(|_| Box::pin(async move { Ok(Some(123)) }));
        mock_card_collection_price_calculation_use_case
            .expect_calculate_total_price()
            .returning(|| Box::pin(async move { Ok(()) }));

        card_repository
            .expect_update_cardmarket_id()
            .returning(|_, _| {
                Box::pin(async move { Err(AppError::RepositoryError("Save failed".to_string())) })
            });

        let service = UpdateCardMarketIdService::new(
            Arc::new(card_repository),
            Arc::new(scryfall_caller),
            Arc::new(mock_card_collection_price_calculation_use_case),
        );

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn handles_empty_card_list() {
        let mut card_repository = MockCardRepository::new();
        let scryfall_caller = MockScryfallCaller::new();
        let mut mock_card_collection_price_calculation_use_case =
            MockCardCollectionPriceCalculationUseCase::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || Box::pin(async move { Ok(vec![]) }));

        mock_card_collection_price_calculation_use_case
            .expect_calculate_total_price()
            .returning(|| Box::pin(async move { Ok(()) }));

        let service = UpdateCardMarketIdService::new(
            Arc::new(card_repository),
            Arc::new(scryfall_caller),
            Arc::new(mock_card_collection_price_calculation_use_case),
        );

        let result = service.update_cards().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn propagates_error_from_card_repository_get_all() {
        let mut card_repository = MockCardRepository::new();
        let scryfall_caller = MockScryfallCaller::new();
        let mut mock_card_collection_price_calculation_use_case =
            MockCardCollectionPriceCalculationUseCase::new();

        card_repository
            .expect_get_all_without_cardmarket_id()
            .returning(move || {
                Box::pin(async move { Err(AppError::RepositoryError("DB error".to_string())) })
            });

        mock_card_collection_price_calculation_use_case
            .expect_calculate_total_price()
            .returning(|| Box::pin(async move { Ok(()) }));

        let service = UpdateCardMarketIdService::new(
            Arc::new(card_repository),
            Arc::new(scryfall_caller),
            Arc::new(mock_card_collection_price_calculation_use_case),
        );

        let result = service.update_cards().await;
        assert!(matches!(
            result,
            Err(AppError::RepositoryError(s)) if s == "DB error"
        ));
    }
}
