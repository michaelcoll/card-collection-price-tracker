use crate::application::error::AppError;
use crate::application::repository::SetNameRepository;
use crate::domain::set_name::{SetCode, SetName};
use crate::infrastructure::adapter_out::repository::entities::SetNameEntity;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct SetNameRepositoryAdapter {
    pool: Pool<Postgres>,
}

impl SetNameRepositoryAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SetNameRepository for SetNameRepositoryAdapter {
    async fn exists_by_code(&self, code: SetCode) -> Result<bool, AppError> {
        Ok(sqlx::query_as!(
            SetNameEntity,
            "SELECT * FROM set_name WHERE set_code = $1",
            code.to_string()
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some())
    }

    async fn save(&self, set: SetName) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO set_name (set_code, name)
             VALUES ($1, $2)
             ON CONFLICT(set_code)
             DO UPDATE
                SET name = $2",
            set.code.to_string(),
            set.name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "set_names_repository_adapter_tests.rs"]
mod tests;
