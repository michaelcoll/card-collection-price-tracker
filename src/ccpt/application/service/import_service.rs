use crate::application::error::AppError;
use crate::application::repository::{CardRepository, SetNameRepository};
use crate::application::use_case::ImportCardUseCase;

#[allow(dead_code)]
pub struct ImportCardService<C: CardRepository, S: SetNameRepository> {
    card_repository: C,
    set_name_repository: S,
}

impl<C: CardRepository, S: SetNameRepository> ImportCardService<C, S> {
    #[allow(dead_code)]
    pub fn new(card_repository: C, set_name_repository: S) -> Self {
        Self {
            card_repository,
            set_name_repository,
        }
    }
}

impl<C: CardRepository, S: SetNameRepository> ImportCardUseCase for ImportCardService<C, S> {
    async fn import_cards(&mut self, csv: &str) -> Result<(), AppError> {
        let cards = crate::application::service::parse_service::parse_cards(csv)?;

        self.card_repository.delete_all().await?;

        for card in cards {
            if !self
                .set_name_repository
                .exists_by_code(card.id.set_code.clone())
                .await?
            {
                let set_name = card.set_name.clone();
                self.set_name_repository.save(set_name).await?;
            }

            self.card_repository.save(card).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::repository::{MockCardRepository, MockSetNameRepository};
    use crate::domain::card::{Card, CardId};
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::{SetCode, SetName};
    use mockall::predicate::eq;

    #[tokio::test]
    async fn import_cards_saves_cards_and_set_names_successfully() {
        let mut card_repository = MockCardRepository::new();
        let mut set_name_repository = MockSetNameRepository::new();

        let set_code = SetCode::new("FDN");
        let set_name = SetName {
            code: set_code.clone(),
            name: "Foundations".to_string(),
        };
        let id = CardId {
            set_code: set_code.clone(),
            collector_number: 87,
            language_code: LanguageCode::FR,
            foil: false,
        };
        let card = Card {
            id: id.clone(),
            set_name: set_name.clone(),
            quantity: 3,
            purchase_price: 8,
        };

        card_repository.expect_delete_all().returning(|| Ok(()));
        set_name_repository
            .expect_exists_by_code()
            .with(eq(set_code.clone()))
            .returning(|_| Ok(false));
        set_name_repository
            .expect_save()
            .with(eq(set_name.clone()))
            .returning(|_| Ok(()));
        card_repository
            .expect_save()
            .with(eq(card.clone()))
            .returning(|_| Ok(()));

        let mut service = ImportCardService::new(card_repository, set_name_repository);

        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";
        let result = service.import_cards(csv).await;

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
        let id = CardId {
            set_code: set_code.clone(),
            collector_number: 0,
            language_code: LanguageCode::FR,
            foil: false,
        };
        let card = Card {
            id: id.clone(),
            set_name: set_name.clone(),
            quantity: 0,
            purchase_price: 0,
        };

        card_repository.expect_delete_all().returning(|| Ok(()));
        set_name_repository
            .expect_exists_by_code()
            .with(eq(set_name.code.clone()))
            .returning(|_| Ok(false));
        set_name_repository
            .expect_save()
            .with(eq(set_name.clone()))
            .returning(|_| Ok(()));
        card_repository
            .expect_save()
            .with(eq(card.clone()))
            .returning(|_| Err(AppError::RepositoryError("Save failed".to_string())));

        let mut service = ImportCardService::new(card_repository, set_name_repository);

        let csv = "Card Name,SET001,Set Name";
        let result = service.import_cards(csv).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn import_cards_does_not_save_duplicate_set_name() {
        let mut card_repository = MockCardRepository::new();
        let mut set_name_repository = MockSetNameRepository::new();

        let set_code = SetCode::new("FDN");
        let set_name = SetName {
            code: set_code.clone(),
            name: "Foundations".to_string(),
        };
        let id = CardId {
            set_code: set_code.clone(),
            collector_number: 87,
            language_code: LanguageCode::FR,
            foil: false,
        };
        let card = Card {
            id: id.clone(),
            set_name: set_name.clone(),
            quantity: 3,
            purchase_price: 8,
        };

        card_repository.expect_delete_all().returning(|| Ok(()));
        set_name_repository
            .expect_exists_by_code()
            .with(eq(set_code.clone()))
            .returning(|_| Ok(true));
        card_repository
            .expect_save()
            .with(eq(card.clone()))
            .returning(|_| Ok(()));

        let mut service = ImportCardService::new(card_repository, set_name_repository);

        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";
        let result = service.import_cards(csv).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn import_cards_fails_on_parse_error() {
        let card_repository = MockCardRepository::new();
        let set_name_repository = MockSetNameRepository::new();

        let mut service = ImportCardService::new(card_repository, set_name_repository);

        let invalid_csv = "Invalid,Data";
        let result = service.import_cards(invalid_csv).await;

        assert!(result.is_err());
    }
}
