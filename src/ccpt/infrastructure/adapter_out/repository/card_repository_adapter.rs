use crate::application::error::AppError;
use crate::application::repository::CardRepository;
use crate::domain::card::{Card, CardId};
use crate::domain::user::User;
use crate::infrastructure::adapter_out::repository::entities::{CardEntity, CardIdEntity};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct CardRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardRepository for CardRepositoryAdapter {
    async fn get_all(&self, user: User) -> Result<Vec<Card>, AppError> {
        Ok(sqlx::query_as!(
            CardEntity,
            "SELECT
                card.*,
                set_name.name as set_name,
                collection_entry.quantity,
                collection_entry.purchase_price,
                collection_entry.added_at
            FROM card
            JOIN set_name ON card.set_code = set_name.set_code
            JOIN collection_entry ON
                card.set_code = collection_entry.set_code AND
                card.collector_number = collection_entry.collector_number AND
                card.language_code = collection_entry.language_code AND
                card.foil = collection_entry.foil AND
                collection_entry.user_id = $1",
            user.id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<Card>>())
    }

    async fn get_all_without_cardmarket_id(&self) -> Result<Vec<(CardId, uuid::Uuid)>, AppError> {
        Ok(sqlx::query_as!(
            CardIdEntity,
            "SELECT
                card.set_code,
                set_name.name as set_name,
                card.collector_number,
                card.language_code,
                card.foil,
                card.scryfall_id
            FROM card
            JOIN set_name ON card.set_code = set_name.set_code
            WHERE card.cardmarket_id IS NULL"
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| (e.clone().into(), e.scryfall_id))
        .collect::<Vec<(CardId, uuid::Uuid)>>())
    }

    async fn save(&self, user: User, card: Card) -> Result<(), AppError> {
        sqlx::query!(
        r#"INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(set_code, collector_number, language_code, foil)
                DO UPDATE
                SET name          = $5,
                    rarity        = $6,
                    scryfall_id   = $7"#,
            card.id.set_code.to_string(),
            card.id.collector_number,
            card.id.language_code.to_string(),
            card.id.foil,
            card.name,
            card.rarity_code.to_string(),
            card.scryfall_id,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
        r#"INSERT INTO collection_entry (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price, added_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT(set_code, collector_number, language_code, foil, user_id)
                DO UPDATE
                SET quantity       = $6,
                    purchase_price = $7,
                    added_at       = $8"#,
            card.id.set_code.to_string(),
            card.id.collector_number,
            card.id.language_code.to_string(),
            card.id.foil,
            user.id,
            card.quantity as i32,
            card.purchase_price as i32,
            card.added_at
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_cardmarket_id(
        &self,
        id: CardId,
        cardmarket_id: Option<u32>,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE card
                SET cardmarket_id = $1
                WHERE set_code = $2 AND collector_number = $3 AND language_code = $4 AND foil = $5;"#,
            cardmarket_id.map(|id| id as i32),
            id.set_code.to_string(),
            id.collector_number,
            id.language_code.to_string(),
            id.foil)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_all(&self, user: User) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM collection_entry WHERE user_id = $1", user.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::language_code::LanguageCode;
    use crate::domain::rarity_code::RarityCode;
    use crate::infrastructure::adapter_out::repository::common_repository_tests::{
        insert_card, insert_card_without_cardmarket_id, insert_collection_entry,
    };
    use chrono::Utc;
    use sqlx::PgPool;

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
        // FDN="Foundations" est déjà seedé par la migration
        insert_card_without_cardmarket_id(&pool, "FDN", "87", "FR", false, "Goblin Boarders").await;
        insert_collection_entry(
            &pool,
            "FDN",
            "87",
            "FR",
            false,
            "test-user-id",
            3,
            500,
            Utc::now(),
        )
        .await;

        let cards = CardRepositoryAdapter::new(pool)
            .get_all(User::for_testing())
            .await
            .unwrap();

        assert_eq!(cards.len(), 1);
        assert_eq!(
            cards[0].id,
            CardId::new("FDN", "87", LanguageCode::FR, false)
        );
        assert_eq!(cards[0].name, "Goblin Boarders");
        assert_eq!(cards[0].quantity, 3);
        assert_eq!(cards[0].purchase_price, 500);
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
        assert_eq!(cards[0].quantity, updated_card.quantity);
        assert_eq!(cards[0].purchase_price, updated_card.purchase_price);
    }

    #[sqlx::test]
    async fn delete_all_removes_all_cards(pool: PgPool) {
        insert_card_without_cardmarket_id(&pool, "FDN", "87", "FR", false, "Goblin Boarders").await;
        insert_card_without_cardmarket_id(&pool, "FDN", "12", "EN", true, "Goblin Boarders").await;
        insert_collection_entry(
            &pool,
            "FDN",
            "87",
            "FR",
            false,
            "test-user-id",
            3,
            500,
            Utc::now(),
        )
        .await;
        insert_collection_entry(
            &pool,
            "FDN",
            "12",
            "EN",
            true,
            "test-user-id",
            2,
            1000,
            Utc::now(),
        )
        .await;

        let repository = CardRepositoryAdapter::new(pool);
        repository.delete_all(User::for_testing()).await.unwrap();

        let cards = repository.get_all(User::for_testing()).await.unwrap();
        assert!(
            cards.is_empty(),
            "all cards should be deleted from the database"
        );
    }

    #[sqlx::test]
    async fn get_all_returns_multiple_cards(pool: PgPool) {
        insert_card_without_cardmarket_id(&pool, "FDN", "87", "FR", false, "Goblin Boarders").await;
        insert_card_without_cardmarket_id(&pool, "FDN", "12", "EN", true, "Goblin Boarders").await;
        insert_collection_entry(
            &pool,
            "FDN",
            "87",
            "FR",
            false,
            "test-user-id",
            3,
            500,
            Utc::now(),
        )
        .await;
        insert_collection_entry(
            &pool,
            "FDN",
            "12",
            "EN",
            true,
            "test-user-id",
            2,
            1000,
            Utc::now(),
        )
        .await;

        let cards = CardRepositoryAdapter::new(pool)
            .get_all(User::for_testing())
            .await
            .unwrap();

        assert_eq!(cards.len(), 2);
        let ids: Vec<&CardId> = cards.iter().map(|c| &c.id).collect();
        assert!(ids.contains(&&CardId::new("FDN", "87", LanguageCode::FR, false)));
        assert!(ids.contains(&&CardId::new("FDN", "12", LanguageCode::EN, true)));
    }

    #[sqlx::test]
    async fn get_all_without_cardmarket_id_returns_only_cards_without_cardmarket_id(pool: PgPool) {
        insert_card_without_cardmarket_id(&pool, "FDN", "87", "FR", false, "Goblin Boarders").await;
        insert_card(&pool, "FDN", "12", "EN", true, "Goblin Boarders", 123).await;
        insert_collection_entry(
            &pool,
            "FDN",
            "87",
            "FR",
            false,
            "test-user-id",
            3,
            500,
            Utc::now(),
        )
        .await;
        insert_collection_entry(
            &pool,
            "FDN",
            "12",
            "EN",
            true,
            "test-user-id",
            2,
            1000,
            Utc::now(),
        )
        .await;

        let cards = CardRepositoryAdapter::new(pool)
            .get_all_without_cardmarket_id()
            .await
            .unwrap();

        assert_eq!(cards.len(), 1);
        assert_eq!(
            cards[0].0,
            CardId::new("FDN", "87", LanguageCode::FR, false)
        );
    }
}
