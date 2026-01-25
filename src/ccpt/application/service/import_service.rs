use crate::application::repository::{CardRepository, SetNameRepository};
use crate::application::service::error::ImportError;
use crate::application::use_case::ImportCardUseCase;

pub struct ImportCardService {
    card_repository: Box<dyn CardRepository>,
    set_name_repository: Box<dyn SetNameRepository>,
}

impl ImportCardService {
    pub fn new(
        card_repository: Box<dyn CardRepository>,
        set_name_repository: Box<dyn SetNameRepository>,
    ) -> Self {
        Self {
            card_repository,
            set_name_repository,
        }
    }
}

impl ImportCardUseCase for ImportCardService {
    fn import_cards(&mut self, csv: &str) -> Result<(), ImportError> {
        let cards = crate::application::service::parse_service::parse_cards(csv)?;

        self.card_repository.delete_all()?;

        for card in cards {
            if !self
                .set_name_repository
                .exists_by_code(card.id.set_code.clone())?
            {
                let set_name = card.set_name.clone();
                self.set_name_repository.save(set_name)?;
            }

            self.card_repository.save(card)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::repository::{
        MockCardRepository, MockSetNameRepository, PersistenceError,
    };
    use crate::domain::card::{Card, CardId};
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::{SetCode, SetName};
    use mockall::predicate::eq;

    #[test]
    fn import_cards_saves_cards_and_set_names_successfully() {
        let mut card_repository = MockCardRepository::new();
        let mut set_name_repository = MockSetNameRepository::new();

        let set_code = SetCode::new("FDN").unwrap();
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

        let mut service =
            ImportCardService::new(Box::new(card_repository), Box::new(set_name_repository));

        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";
        let result = service.import_cards(csv);

        assert!(result.is_ok());
    }

    #[test]
    fn import_cards_rollback_on_card_save_error() {
        let mut card_repository = MockCardRepository::new();
        let mut set_name_repository = MockSetNameRepository::new();

        let set_code = SetCode::new("FDN").unwrap();
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
            .returning(|_| Err(PersistenceError::SaveError("Save failed".to_string())));

        let mut service =
            ImportCardService::new(Box::new(card_repository), Box::new(set_name_repository));

        let csv = "Card Name,SET001,Set Name";
        let result = service.import_cards(csv);

        assert!(result.is_err());
    }

    #[test]
    fn import_cards_does_not_save_duplicate_set_name() {
        let mut card_repository = MockCardRepository::new();
        let mut set_name_repository = MockSetNameRepository::new();

        let set_code = SetCode::new("FDN").unwrap();
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

        let mut service =
            ImportCardService::new(Box::new(card_repository), Box::new(set_name_repository));

        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";
        let result = service.import_cards(csv);

        assert!(result.is_ok());
    }

    #[test]
    fn import_cards_fails_on_parse_error() {
        let card_repository = MockCardRepository::new();
        let set_name_repository = MockSetNameRepository::new();

        let mut service =
            ImportCardService::new(Box::new(card_repository), Box::new(set_name_repository));

        let invalid_csv = "Invalid,Data";
        let result = service.import_cards(invalid_csv);

        assert!(result.is_err());
    }
}
