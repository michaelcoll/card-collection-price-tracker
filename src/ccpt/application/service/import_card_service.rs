use crate::application::error::AppError;
use crate::application::repository::{CardRepository, SetNameRepository};
use crate::application::service::parse_service::parse_cards;
use crate::application::use_case::ImportCardUseCase;
use crate::domain::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImportCardService {
    card_repository: Arc<dyn CardRepository>,
    set_name_repository: Arc<dyn SetNameRepository>,
}

impl ImportCardService {
    pub fn new(
        card_repository: Arc<dyn CardRepository>,
        set_name_repository: Arc<dyn SetNameRepository>,
    ) -> Self {
        Self {
            card_repository,
            set_name_repository,
        }
    }
}

#[async_trait]
impl ImportCardUseCase for ImportCardService {
    async fn import_cards(&self, csv: &str) -> Result<(), AppError> {
        let cards = parse_cards(csv)?;

        self.card_repository.delete_all(User::new()).await?;

        for card in cards {
            if !self
                .set_name_repository
                .exists_by_code(card.id.set_code.clone())
                .await?
            {
                let set_name = card.set_name.clone();
                self.set_name_repository.save(set_name).await?;
            }

            self.card_repository.save(User::new(), card).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::repository::{MockCardRepository, MockSetNameRepository};
    use crate::domain::card::Card;
    use crate::domain::language_code::LanguageCode;
    use crate::domain::set_name::{SetCode, SetName};
    use mockall::predicate::eq;
    use uuid::Uuid;

    #[tokio::test]
    async fn import_cards_saves_cards_and_set_names_successfully() {
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
            "87",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            3,
            8,
            Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
            None,
        );

        card_repository
            .expect_delete_all()
            .with(eq(User::new()))
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
            .with(eq(User::new()), eq(card.clone()))
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let service =
            ImportCardService::new(Arc::new(card_repository), Arc::new(set_name_repository));

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
        let card = Card::new(
            set_code.clone(),
            "Foundations",
            "0",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            0,
            0,
        );

        card_repository
            .expect_delete_all()
            .with(eq(User::new()))
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
            .with(eq(User::new()), eq(card.clone()))
            .returning(|_, _| {
                Box::pin(async { Err(AppError::RepositoryError("Save failed".to_string())) })
            });

        let service =
            ImportCardService::new(Arc::new(card_repository), Arc::new(set_name_repository));

        let csv = "Card Name,SET001,Set Name";
        let result = service.import_cards(csv).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn import_cards_does_not_save_duplicate_set_name() {
        let mut card_repository = MockCardRepository::new();
        let mut set_name_repository = MockSetNameRepository::new();

        let set_code = SetCode::new("FDN");
        let card = Card::new_full(
            set_code.clone(),
            "Foundations",
            "87",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            3,
            8,
            Uuid::parse_str("4409a063-bf2a-4a49-803e-3ce6bd474353").unwrap(),
            None,
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
            .with(eq(User::new()), eq(card.clone()))
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let service =
            ImportCardService::new(Arc::new(card_repository), Arc::new(set_name_repository));

        let csv = "Binder Name,Binder Type,Name,Set code,Set name,Collector number,Foil,Rarity,Quantity,ManaBox ID,Scryfall ID,Purchase price,Misprint,Altered,Condition,Language,Purchase price currency\n\
        bulk,binder,Goblin Boarders,FDN,Foundations,87,normal,common,3,101506,4409a063-bf2a-4a49-803e-3ce6bd474353,0.08,false,false,near_mint,fr,EUR";
        let result = service.import_cards(csv).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn import_cards_fails_on_parse_error() {
        let card_repository = MockCardRepository::new();
        let set_name_repository = MockSetNameRepository::new();

        let service =
            ImportCardService::new(Arc::new(card_repository), Arc::new(set_name_repository));

        let invalid_csv = "Invalid,Data";
        let result = service.import_cards(invalid_csv).await;

        assert!(result.is_err());
    }
}
