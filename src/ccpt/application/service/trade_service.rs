use crate::application::error::AppError;
use crate::application::repository::TradeRepository;
use crate::application::use_case::CreateTradeUseCase;
use crate::domain::card::CardId;
use crate::domain::error::FunctionalError;
use crate::domain::trade::{TradeId, TradeStatus};
use crate::domain::user::UserId;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CreateTradeService {
    trade_repository: Arc<dyn TradeRepository>,
    creation_lock: Mutex<()>,
}

impl CreateTradeService {
    pub fn new(trade_repository: Arc<dyn TradeRepository>) -> Self {
        Self {
            trade_repository,
            creation_lock: Mutex::new(()),
        }
    }
}

#[async_trait]
impl CreateTradeUseCase for CreateTradeService {
    async fn create_trade(
        &self,
        initiator_user_id: UserId,
        respondent_user_id: UserId,
        card_id: CardId,
        quantity: u8,
    ) -> Result<TradeId, AppError> {
        let _guard = self.creation_lock.lock().await;

        let owned_quantity = self
            .trade_repository
            .find_collection_entry_quantity(&respondent_user_id, &card_id)
            .await?;
        match owned_quantity {
            Some(q) if q >= quantity as i32 => {}
            _ => return Err(FunctionalError::CardNotFound.into()),
        }

        if respondent_user_id == initiator_user_id {
            return Err(FunctionalError::SelfTrade.into());
        }

        let active_trade = self
            .trade_repository
            .find_active_trade(&initiator_user_id, &respondent_user_id)
            .await?;

        match active_trade {
            None => {
                let id = TradeId::new();
                self.trade_repository
                    .create(
                        id,
                        &initiator_user_id,
                        &respondent_user_id,
                        &card_id,
                        quantity,
                    )
                    .await?;
                Ok(id)
            }
            Some((trade_id, TradeStatus::Pending)) => {
                self.trade_repository
                    .merge_card_into_trade(trade_id, &card_id, &respondent_user_id, quantity, false)
                    .await?;
                Ok(trade_id)
            }
            Some((trade_id, TradeStatus::OneAccepted)) => {
                self.trade_repository
                    .merge_card_into_trade(trade_id, &card_id, &respondent_user_id, quantity, true)
                    .await?;
                Ok(trade_id)
            }
            Some((_, TradeStatus::FullyAccepted)) => {
                Err(FunctionalError::TradeNotModifiable.into())
            }
            Some((_, TradeStatus::Completed | TradeStatus::Closed | TradeStatus::Abandoned)) => {
                unreachable!(
                    "find_active_trade only returns PENDING, ONE_ACCEPTED or FULLY_ACCEPTED trades"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::repository::MockTradeRepository;
    use crate::domain::language_code::LanguageCode;

    fn make_initiator_id() -> UserId {
        UserId::new("user_initiator")
    }

    fn make_respondent_id() -> UserId {
        UserId::new("user_respondent")
    }

    fn make_card_id() -> CardId {
        CardId::new("FDN", "87", LanguageCode::FR, false)
    }

    #[tokio::test]
    async fn create_trade_creates_new_trade_when_no_active_trade_exists() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(3)) }));
        mock_repository
            .expect_find_active_trade()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(None) }));
        mock_repository
            .expect_create()
            .times(1)
            .returning(|_, _, _, _, _| Box::pin(async { Ok(()) }));

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_trade_fails_when_respondent_unknown_or_does_not_own_card() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(None) }));

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
            .await;

        assert!(matches!(
            result,
            Err(AppError::Functional(FunctionalError::CardNotFound))
        ));
    }

    #[tokio::test]
    async fn create_trade_fails_when_respondent_owns_zero() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(0)) }));

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
            .await;

        assert!(matches!(
            result,
            Err(AppError::Functional(FunctionalError::CardNotFound))
        ));
    }

    #[tokio::test]
    async fn create_trade_fails_when_owned_quantity_is_insufficient() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(2)) }));

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 3)
            .await;

        assert!(matches!(
            result,
            Err(AppError::Functional(FunctionalError::CardNotFound))
        ));
    }

    #[tokio::test]
    async fn create_trade_fails_on_self_targeting() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(1)) }));

        let initiator_id = make_initiator_id();
        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(initiator_id.clone(), initiator_id, make_card_id(), 1)
            .await;

        assert!(matches!(
            result,
            Err(AppError::Functional(FunctionalError::SelfTrade))
        ));
    }

    #[tokio::test]
    async fn create_trade_merges_into_pending_trade_without_reopening() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(1)) }));

        let existing_id = TradeId::new();
        mock_repository
            .expect_find_active_trade()
            .times(1)
            .returning(move |_, _| {
                Box::pin(async move { Ok(Some((existing_id, TradeStatus::Pending))) })
            });
        mock_repository
            .expect_merge_card_into_trade()
            .times(1)
            .withf(move |trade_id, _, _, _, reopen| *trade_id == existing_id && !*reopen)
            .returning(|_, _, _, _, _| Box::pin(async { Ok(()) }));

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
            .await;

        assert_eq!(result.unwrap(), existing_id);
    }

    #[tokio::test]
    async fn create_trade_merges_into_one_accepted_trade_and_reopens_it() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(1)) }));

        let existing_id = TradeId::new();
        mock_repository
            .expect_find_active_trade()
            .times(1)
            .returning(move |_, _| {
                Box::pin(async move { Ok(Some((existing_id, TradeStatus::OneAccepted))) })
            });
        mock_repository
            .expect_merge_card_into_trade()
            .times(1)
            .withf(move |trade_id, _, _, _, reopen| *trade_id == existing_id && *reopen)
            .returning(|_, _, _, _, _| Box::pin(async { Ok(()) }));

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
            .await;

        assert_eq!(result.unwrap(), existing_id);
    }

    #[tokio::test]
    async fn create_trade_fails_when_active_trade_is_fully_accepted() {
        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(Some(1)) }));
        mock_repository
            .expect_find_active_trade()
            .times(1)
            .returning(|_, _| {
                Box::pin(async { Ok(Some((TradeId::new(), TradeStatus::FullyAccepted))) })
            });

        let service = CreateTradeService::new(Arc::new(mock_repository));
        let result = service
            .create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
            .await;

        assert!(matches!(
            result,
            Err(AppError::Functional(FunctionalError::TradeNotModifiable))
        ));
    }

    #[tokio::test]
    async fn create_trade_lock_serializes_concurrent_creations() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let created = Arc::new(AtomicBool::new(false));

        let mut mock_repository = MockTradeRepository::new();
        mock_repository
            .expect_find_collection_entry_quantity()
            .times(2)
            .returning(|_, _| Box::pin(async { Ok(Some(1)) }));

        let created_for_find = created.clone();
        mock_repository
            .expect_find_active_trade()
            .times(2)
            .returning(move |_, _| {
                let found = created_for_find.load(Ordering::SeqCst);
                Box::pin(async move { Ok(found.then(|| (TradeId::new(), TradeStatus::Pending))) })
            });

        let created_for_create = created.clone();
        mock_repository
            .expect_create()
            .times(1)
            .returning(move |_, _, _, _, _| {
                created_for_create.store(true, Ordering::SeqCst);
                Box::pin(async { Ok(()) })
            });

        mock_repository
            .expect_merge_card_into_trade()
            .times(1)
            .returning(|_, _, _, _, _| Box::pin(async { Ok(()) }));

        let service = Arc::new(CreateTradeService::new(Arc::new(mock_repository)));

        let service_a = service.clone();
        let service_b = service.clone();
        let (result_a, result_b) = tokio::join!(
            service_a.create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1),
            service_b.create_trade(make_initiator_id(), make_respondent_id(), make_card_id(), 1)
        );

        assert!(result_a.is_ok());
        assert!(result_b.is_ok());
    }
}
