use crate::application::error::AppError;
use crate::application::repository::CardRepository;
use crate::domain::card::Card;
use crate::domain::user::User;
use crate::infrastructure::adapter_out::repository::entities::CardEntity;
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
                card_quantity.quantity,
                card_quantity.purchase_price
            FROM card
            JOIN set_name ON card.set_code = set_name.set_code
            JOIN card_quantity ON
                card.set_code = card_quantity.set_code AND
                card.collector_number = card_quantity.collector_number AND
                card.language_code = card_quantity.language_code AND
                card.foil = card_quantity.foil AND
                card_quantity.user_id = $1",
            user.id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<Card>>())
    }

    async fn save(&self, user: User, card: Card) -> Result<(), AppError> {
        sqlx::query!(
            "
            INSERT INTO card (set_code, collector_number, language_code, foil, name, scryfall_id, cardmarket_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(set_code, collector_number, language_code, foil)
                DO NOTHING",
            card.id.set_code.to_string(),
            card.id.collector_number,
            card.id.language_code.to_string(),
            card.id.foil,
            card.name,
            card.scryfall_id,
            card.cardmarket_id.map(|id| id as i32)
        )
            .execute(&self.pool)
            .await?;

        sqlx::query!("
            INSERT INTO card_quantity (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(set_code, collector_number, language_code, foil, user_id)
                DO UPDATE
                SET quantity       = $6,
                    purchase_price = $7",
            card.id.set_code.to_string(),
            card.id.collector_number,
            card.id.language_code.to_string(),
            card.id.foil,
            user.id,
            card.quantity as i32,
            card.purchase_price as i32
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_all(&self, user: User) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM card_quantity WHERE user_id = $1", user.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::language_code::LanguageCode;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_no_card_exists(pool: PgPool) {
        let vec = CardRepositoryAdapter::new(pool)
            .get_all(User::new())
            .await
            .unwrap();
        assert!(vec.is_empty(), "no cards should exist in the database");
    }

    #[sqlx::test]
    async fn test_get_user_id(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card = Card::new(
            "FDN",
            "Foundations",
            "87",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            3,
            500,
        );

        repository.save(User::new(), card.clone()).await.unwrap();

        let cards = repository.get_all(User::new()).await.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], card);
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
            3,
            500,
        );
        repository.save(User::new(), card.clone()).await.unwrap();

        let updated_card = Card::new(
            "FDN",
            "Foundations",
            "87",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            5,
            1500,
        );
        repository
            .save(User::new(), updated_card.clone())
            .await
            .unwrap();

        let cards = repository.get_all(User::new()).await.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], updated_card);
    }

    #[sqlx::test]
    async fn delete_all_removes_all_cards(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card1 = Card::new(
            "FDN",
            "Foundations",
            "87",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            3,
            500,
        );
        let card2 = Card::new(
            "FDN",
            "Foundations",
            "12",
            LanguageCode::EN,
            true,
            "Goblin Boarders",
            2,
            1000,
        );

        repository.save(User::new(), card1).await.unwrap();
        repository.save(User::new(), card2).await.unwrap();

        repository.delete_all(User::new()).await.unwrap();

        let cards = repository.get_all(User::new()).await.unwrap();
        assert!(
            cards.is_empty(),
            "all cards should be deleted from the database"
        );
    }

    #[sqlx::test]
    async fn get_all_returns_multiple_cards(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card1 = Card::new(
            "FDN",
            "Foundations",
            "87",
            LanguageCode::FR,
            false,
            "Goblin Boarders",
            3,
            500,
        );
        let card2 = Card::new(
            "FDN",
            "Foundations",
            "12",
            LanguageCode::EN,
            true,
            "Goblin Boarders",
            2,
            1000,
        );

        repository.save(User::new(), card1.clone()).await.unwrap();
        repository.save(User::new(), card2.clone()).await.unwrap();

        let cards = repository.get_all(User::new()).await.unwrap();
        assert_eq!(cards.len(), 2);
        assert!(cards.contains(&card1));
        assert!(cards.contains(&card2));
    }
}
