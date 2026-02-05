use crate::application::error::AppError;
use crate::application::repository::CardRepository;
use crate::domain::card::Card;
use crate::infrastructure::adapter_out::entities::CardEntity;
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
    async fn get_all(&self) -> Result<Vec<Card>, AppError> {
        Ok(sqlx::query_as!(
            CardEntity,
            "SELECT
                card.*,
                set_name.name as set_name
            FROM card
            JOIN set_name ON card.set_code = set_name.set_code"
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<Card>>())
    }

    async fn save(&self, card: Card) -> Result<(), AppError> {
        sqlx::query!("
            INSERT INTO card (set_code, collector_number, language_code, foil, quantity, purchase_price)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT(set_code, collector_number, language_code, foil)
                DO UPDATE
                SET quantity       = $5,
                    purchase_price = $6",
            card.id.set_code.to_string(),
            card.id.collector_number as i32,
            card.id.language_code.to_string(),
            card.id.foil,
            card.quantity as i32,
            card.purchase_price as i32)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_all(&self) -> Result<(), AppError> {
        sqlx::query!("TRUNCATE TABLE card")
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
        let vec = CardRepositoryAdapter::new(pool).get_all().await.unwrap();
        assert!(vec.is_empty(), "no cards should exist in the database");
    }

    #[sqlx::test]
    async fn test_get_user_id(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card = Card::new("FDN", "Foundations", 87, LanguageCode::FR, false, 3, 500);

        repository.save(card.clone()).await.unwrap();

        let cards = repository.get_all().await.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], card);
    }

    #[sqlx::test]
    async fn save_card_updates_existing_card(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card = Card::new("FDN", "Foundations", 87, LanguageCode::FR, false, 3, 500);
        repository.save(card.clone()).await.unwrap();

        let updated_card = Card::new("FDN", "Foundations", 87, LanguageCode::FR, false, 5, 1500);
        repository.save(updated_card.clone()).await.unwrap();

        let cards = repository.get_all().await.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], updated_card);
    }

    #[sqlx::test]
    async fn delete_all_removes_all_cards(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card1 = Card::new("FDN", "Foundations", 87, LanguageCode::FR, false, 3, 500);
        let card2 = Card::new("FDN", "Foundations", 12, LanguageCode::EN, true, 2, 1000);

        repository.save(card1).await.unwrap();
        repository.save(card2).await.unwrap();

        repository.delete_all().await.unwrap();

        let cards = repository.get_all().await.unwrap();
        assert!(
            cards.is_empty(),
            "all cards should be deleted from the database"
        );
    }

    #[sqlx::test]
    async fn get_all_returns_multiple_cards(pool: PgPool) {
        let repository = CardRepositoryAdapter::new(pool);

        let card1 = Card::new("FDN", "Foundations", 87, LanguageCode::FR, false, 3, 500);
        let card2 = Card::new("FDN", "Foundations", 12, LanguageCode::EN, true, 2, 1000);

        repository.save(card1.clone()).await.unwrap();
        repository.save(card2.clone()).await.unwrap();

        let cards = repository.get_all().await.unwrap();
        assert_eq!(cards.len(), 2);
        assert!(cards.contains(&card1));
        assert!(cards.contains(&card2));
    }
}
