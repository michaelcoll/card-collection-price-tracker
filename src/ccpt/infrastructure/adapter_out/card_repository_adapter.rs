use crate::application::error::AppError;
use crate::application::repository::{CardRepository, PersistenceError};
use crate::domain::card::Card;
use crate::infrastructure::adapter_out::entities::CardEntity;
use sqlx::{Error, Pool, Postgres};

pub struct CardRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl CardRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl From<Error> for PersistenceError {
    fn from(_err: Error) -> Self {
        PersistenceError::DBError(_err.to_string())
    }
}

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

    async fn save(&mut self, card: Card) -> Result<(), AppError> {
        sqlx::query!("
            INSERT INTO card (set_code, collector_number, language_code, foil, quantity, purchase_price)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT(set_code, collector_number, language_code, foil)
                DO UPDATE
                SET quantity       = 5,
                    purchase_price = 15",
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

    async fn delete_all(&mut self) -> Result<(), AppError> {
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

    // if you want you can specify a different migrations directory
    #[sqlx::test]
    async fn test_get_user_id(pool: PgPool) {
        let mut repository = CardRepositoryAdapter::new(pool);

        let card = Card::new("FDN", "Foundations", 87, LanguageCode::FR, false, 3, 500);

        repository.save(card.clone()).await.unwrap();

        let cards = repository.get_all().await.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], card);
    }
}
