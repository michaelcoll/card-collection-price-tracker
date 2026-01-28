use crate::application::caller::CardPriceCaller;
use crate::application::error::AppError;
use crate::application::repository::{CardCollectionRepository, CardRepository};
use crate::application::use_case::CardCollectionPriceCalculationUseCase;
use crate::domain::price::Price;

#[allow(dead_code)]
pub struct CardCollectionService<
    CPC: CardPriceCaller,
    CR: CardRepository,
    CCR: CardCollectionRepository,
> {
    card_price_caller: CPC,
    card_repository: CR,
    card_collection_repository: CCR,
}

impl<CPC: CardPriceCaller, CR: CardRepository, CCR: CardCollectionRepository>
    CardCollectionService<CPC, CR, CCR>
{
    #[allow(dead_code)]
    pub fn new(
        card_price_caller: CPC,
        card_repository: CR,
        card_collection_repository: CCR,
    ) -> Self {
        Self {
            card_price_caller,
            card_repository,
            card_collection_repository,
        }
    }
}

impl<CPC: CardPriceCaller, CR: CardRepository, CCR: CardCollectionRepository>
    CardCollectionPriceCalculationUseCase for CardCollectionService<CPC, CR, CCR>
{
    async fn calculate_total_price(&mut self) -> Result<(), AppError> {
        let cards = self.card_repository.get_all().await?;

        let mut total_price: Price = Price::zero();

        for card in cards {
            let price = self
                .card_price_caller
                .get_price_by_card_id(card.id.clone())?;
            total_price += price;
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
        let card_id1 = CardId {
            set_code: set_code.clone(),
            collector_number: 0,
            language_code: LanguageCode::FR,
            foil: false,
        };
        let card_id2 = CardId {
            set_code: set_code.clone(),
            collector_number: 1,
            language_code: LanguageCode::FR,
            foil: false,
        };

        let card1 = Card {
            id: card_id1.clone(),
            set_name: set_name.clone(),
            quantity: 1,
            purchase_price: 2,
        };
        let card2 = Card {
            id: card_id2.clone(),
            set_name: set_name.clone(),
            quantity: 1,
            purchase_price: 2,
        };

        let cards = vec![card1.clone(), card2.clone()];

        card_repository
            .expect_get_all()
            .returning(move || Ok(cards.clone()));

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id1.clone()))
            .returning(|_| {
                Ok(Price {
                    date: Default::default(),
                    low: 100,
                    trend: 200,
                    avg1: 300,
                    avg7: 400,
                    avg30: 500,
                })
            });

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id2.clone()))
            .returning(|_| {
                Ok(Price {
                    date: Default::default(),
                    low: 50,
                    trend: 100,
                    avg1: 150,
                    avg7: 200,
                    avg30: 250,
                })
            });

        card_collection_repository
            .expect_save()
            .with(eq(Price {
                date: Default::default(),
                low: 150,
                trend: 300,
                avg1: 450,
                avg7: 600,
                avg30: 750,
            }))
            .returning(|_| Ok(()));

        let mut service = CardCollectionService::new(
            card_price_caller,
            card_repository,
            card_collection_repository,
        );

        let result = service.calculate_total_price().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn calculate_total_price_handles_empty_card_list() {
        let card_price_caller = MockCardPriceCaller::new();
        let mut card_repository = MockCardRepository::new();
        let mut card_collection_repository = MockCardCollectionRepository::new();

        card_repository.expect_get_all().returning(|| Ok(vec![]));

        card_collection_repository
            .expect_save()
            .with(eq(Price::zero()))
            .returning(|_| Ok(()));

        let mut service = CardCollectionService::new(
            card_price_caller,
            card_repository,
            card_collection_repository,
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
            .returning(|| Err(RepositoryError("DB error".to_string())));

        let mut service = CardCollectionService::new(
            card_price_caller,
            card_repository,
            card_collection_repository,
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
        let card_id1 = CardId {
            set_code: set_code.clone(),
            collector_number: 0,
            language_code: LanguageCode::FR,
            foil: false,
        };

        let card1 = Card {
            id: card_id1.clone(),
            set_name: set_name.clone(),
            quantity: 1,
            purchase_price: 2,
        };

        let cards = vec![card1.clone()];

        card_repository
            .expect_get_all()
            .returning(move || Ok(cards.clone()));

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id1.clone()))
            .returning(|_| Err(PriceNotFound));

        let mut service = CardCollectionService::new(
            card_price_caller,
            card_repository,
            card_collection_repository,
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
        let card_id1 = CardId {
            set_code: set_code.clone(),
            collector_number: 0,
            language_code: LanguageCode::FR,
            foil: false,
        };

        let card1 = Card {
            id: card_id1.clone(),
            set_name: set_name.clone(),
            quantity: 1,
            purchase_price: 2,
        };
        let cards = vec![card1.clone()];

        card_repository
            .expect_get_all()
            .returning(move || Ok(cards.clone()));

        card_price_caller
            .expect_get_price_by_card_id()
            .with(eq(card_id1.clone()))
            .returning(|_| {
                Ok(Price {
                    date: Default::default(),
                    low: 100,
                    trend: 200,
                    avg1: 300,
                    avg7: 400,
                    avg30: 500,
                })
            });

        card_collection_repository
            .expect_save()
            .with(eq(Price {
                date: Default::default(),
                low: 100,
                trend: 200,
                avg1: 300,
                avg7: 400,
                avg30: 500,
            }))
            .returning(|_| Err(RepositoryError("DB error".to_string())));

        let mut service = CardCollectionService::new(
            card_price_caller,
            card_repository,
            card_collection_repository,
        );

        let result = service.calculate_total_price().await;
        assert!(matches!(
            result,
            Err(AppError::RepositoryError(s)) if s == "DB error"
        ));
    }
}
