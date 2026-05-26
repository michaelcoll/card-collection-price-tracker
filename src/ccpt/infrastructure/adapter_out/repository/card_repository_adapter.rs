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
            "
            INSERT INTO card (set_code, collector_number, language_code, foil, name, rarity, scryfall_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(set_code, collector_number, language_code, foil)
                DO UPDATE
                SET name          = $5,
                    rarity        = $6,
                    scryfall_id   = $7",
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

        sqlx::query!("
            INSERT INTO collection_entry (set_code, collector_number, language_code, foil, user_id, quantity, purchase_price, added_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT(set_code, collector_number, language_code, foil, user_id)
                DO UPDATE
                SET quantity       = $6,
                    purchase_price = $7,
                    added_at       = $8",
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
        sqlx::query!("
                UPDATE card
                SET cardmarket_id = $1
                WHERE set_code = $2 AND collector_number = $3 AND language_code = $4 AND foil = $5;",
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
#[path = "card_repository_adapter_tests.rs"]
mod tests;
