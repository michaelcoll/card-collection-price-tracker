use crate::application::error::AppError;
use crate::application::repository::CollectionRepository;
use crate::application::use_case::GetCollectionUseCase;
use crate::domain::collection::{CollectionQuery, PaginatedCollection};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CollectionService {
    repository: Arc<dyn CollectionRepository>,
}

impl CollectionService {
    pub fn new(repository: Arc<dyn CollectionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl GetCollectionUseCase for CollectionService {
    async fn get_collection(
        &self,
        user_id: &str,
        query: CollectionQuery,
    ) -> Result<PaginatedCollection, AppError> {
        self.repository.get_paginated(user_id, query).await
    }
}

#[cfg(test)]
#[path = "collection_service_tests.rs"]
mod tests;
