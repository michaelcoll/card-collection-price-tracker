use crate::application::caller::CardPriceCaller;
use crate::application::error::AppError;
use crate::application::repository::{CardCollectionRepository, CardRepository};
use crate::application::use_case::CardCollectionPriceCalculationUseCase;
use crate::domain::price::PriceGuide;
use crate::domain::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CardCollectionService {
    card_price_caller: Arc<dyn CardPriceCaller>,
    card_repository: Arc<dyn CardRepository>,
    card_collection_repository: Arc<dyn CardCollectionRepository>,
}

impl CardCollectionService {
    pub fn new(
        card_price_caller: Arc<dyn CardPriceCaller>,
        card_repository: Arc<dyn CardRepository>,
        card_collection_repository: Arc<dyn CardCollectionRepository>,
    ) -> Self {
        Self {
            card_price_caller,
            card_repository,
            card_collection_repository,
        }
    }
}

#[async_trait]
impl CardCollectionPriceCalculationUseCase for CardCollectionService {
    async fn calculate_total_price(&self) -> Result<(), AppError> {
        let cards = &self.card_repository.get_all(User::new()).await?;

        let mut total_price = PriceGuide::empty();

        for card in cards {
            let price = &self
                .card_price_caller
                .get_price_by_card_id(card.id.clone())
                .await?;
            total_price += price.clone();
        }

        self.card_collection_repository.save(total_price).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::caller::MockCardPriceCaller;
    use crate::application::error::AppError::{PriceNotFound, RepositoryError};
    use crate::application::repository::{MockCardCollectionRepository, MockCardRepository};
    use crate::domain::card::{Card, CardId};
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::{SetCode, SetName};
    use mockall::predicate::*;

    #[tokio::test]
    async fn calculate_total_price_saves_correct_total() {
        let mut card_price_caller = MockCardPriceCaller::new();
        let mut card_repository = MockCardRepository::new();
        let mut card_collection_repository = MockCardCollectionRepository::new();

        let set_code = SetCode::new("FDN");
        let set_name = SetName {
            code: set_code.clone(),
            name: "Foundations".to_string(),
        };
        let card_id1 = CardId::new("FDN", "0", LanguageCode::FR, false);
        let card_id2 = CardId::new("FDN", "1", LanguageCode::FR, false);

        let card1 = Card {
            id: card_id1.clone(),
            set_name: set_name.clone(),
            name: "Goblin Boarders".to_string(),
            quantity: 1,
            purchase_price: 2,
        };
        let card2 = Card {
            id: card_id2.clone(),
            set_name: set_name.clone(),
            name: "Goblin Boarders".to_string(),
            quantity: 1,
            purchase_price: 2,
        };

        let cards = vec![card1.clone(), card2.clone()];
        let cards = Arc::new(cards);

        card_repository
            .expect_get_all()
            .with(eq(User::new()))
            .returning({
                let cards = cards.clone();
                move |_| {
                    Box::pin({
                        let value = cards.clone();
                        async move { Ok(value.as_ref().clone()) }
                    })
                }
            });

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id1.clone()))
            .returning(|_| {
                Box::pin(async move { Ok(PriceGuide::new(100, 200, 300, 400, 500, 600)) })
            });

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id2.clone()))
            .returning(|_| {
                Box::pin(async move { Ok(PriceGuide::new(50, 100, 150, 200, 250, 300)) })
            });

        card_collection_repository
            .expect_save()
            .with(eq(PriceGuide::new(150, 300, 450, 600, 750, 900)))
            .returning(|_| Box::pin(async move { Ok(()) }));

        let service = CardCollectionService::new(
            Arc::new(card_price_caller),
            Arc::new(card_repository),
            Arc::new(card_collection_repository),
        );

        let result = service.calculate_total_price().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn calculate_total_price_handles_empty_card_list() {
        let card_price_caller = MockCardPriceCaller::new();
        let mut card_repository = MockCardRepository::new();
        let mut card_collection_repository = MockCardCollectionRepository::new();

        card_repository
            .expect_get_all()
            .with(eq(User::new()))
            .returning(|_| Box::pin(async move { Ok(vec![]) }));

        card_collection_repository
            .expect_save()
            .with(eq(PriceGuide::empty()))
            .returning(|_| Box::pin(async move { Ok(()) }));

        let service = CardCollectionService::new(
            Arc::new(card_price_caller),
            Arc::new(card_repository),
            Arc::new(card_collection_repository),
        );

        let result = service.calculate_total_price().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn calculate_total_price_propagates_error_from_card_repository() {
        let card_price_caller = MockCardPriceCaller::new();
        let mut card_repository = MockCardRepository::new();
        let card_collection_repository = MockCardCollectionRepository::new();

        card_repository
            .expect_get_all()
            .with(eq(User::new()))
            .returning(|_| Box::pin(async move { Err(RepositoryError("DB error".to_string())) }));

        let service = CardCollectionService::new(
            Arc::new(card_price_caller),
            Arc::new(card_repository),
            Arc::new(card_collection_repository),
        );

        let result = service.calculate_total_price().await;
        assert!(matches!(
            result,
            Err(AppError::RepositoryError(s)) if s == "DB error"
        ));
    }

    #[tokio::test]
    async fn calculate_total_price_propagates_error_from_price_caller() {
        let mut card_price_caller = MockCardPriceCaller::new();
        let mut card_repository = MockCardRepository::new();
        let card_collection_repository = MockCardCollectionRepository::new();

        let set_code = SetCode::new("FDN");
        let set_name = SetName {
            code: set_code.clone(),
            name: "Foundations".to_string(),
        };
        let card_id1 = CardId::new("FDN", "0", LanguageCode::FR, false);

        let card1 = Card {
            id: card_id1.clone(),
            set_name: set_name.clone(),
            name: "Goblin Boarders".to_string(),
            quantity: 1,
            purchase_price: 2,
        };

        let cards = vec![card1.clone()];
        let cards = Arc::new(cards);

        card_repository
            .expect_get_all()
            .with(eq(User::new()))
            .returning({
                let cards = cards.clone();
                move |_| {
                    Box::pin({
                        let value = cards.clone();
                        async move { Ok(value.as_ref().clone()) }
                    })
                }
            });

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id1.clone()))
            .returning(|_| Box::pin(async move { Err(PriceNotFound) }));

        let service = CardCollectionService::new(
            Arc::new(card_price_caller),
            Arc::new(card_repository),
            Arc::new(card_collection_repository),
        );

        let result = service.calculate_total_price().await;
        assert!(matches!(result, Err(PriceNotFound)));
    }

    #[tokio::test]
    async fn calculate_total_price_propagates_error_from_collection_repository() {
        let mut card_price_caller = MockCardPriceCaller::new();
        let mut card_repository = MockCardRepository::new();
        let mut card_collection_repository = MockCardCollectionRepository::new();

        let set_code = SetCode::new("FDN");
        let set_name = SetName {
            code: set_code.clone(),
            name: "Foundations".to_string(),
        };
        let card_id1 = CardId::new("FDN", "0", LanguageCode::FR, false);

        let card1 = Card {
            id: card_id1.clone(),
            set_name: set_name.clone(),
            name: "Goblin Boarders".to_string(),
            quantity: 1,
            purchase_price: 2,
        };
        let cards = vec![card1.clone()];
        let cards = Arc::new(cards);

        card_repository
            .expect_get_all()
            .with(eq(User::new()))
            .returning({
                let cards = cards.clone();
                move |_| {
                    Box::pin({
                        let value = cards.clone();
                        async move { Ok(value.as_ref().clone()) }
                    })
                }
            });

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id1.clone()))
            .returning(|_| {
                Box::pin(async move { Ok(PriceGuide::new(100, 200, 300, 400, 500, 600)) })
            });

        card_collection_repository
            .expect_save()
            .with(eq(PriceGuide::new(100, 200, 300, 400, 500, 600)))
            .returning(|_| Box::pin(async move { Err(RepositoryError("DB error".to_string())) }));

        let service = CardCollectionService::new(
            Arc::new(card_price_caller),
            Arc::new(card_repository),
            Arc::new(card_collection_repository),
        );

        let result = service.calculate_total_price().await;
        assert!(matches!(
            result,
            Err(RepositoryError(s)) if s == "DB error"
        ));
    }
}
