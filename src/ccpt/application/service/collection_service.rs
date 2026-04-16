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
mod tests {
    use super::*;
    use crate::application::repository::MockCollectionRepository;
    use crate::domain::collection::{CollectionSortField, SortDirection};

    #[tokio::test]
    async fn get_collection_delegates_to_repository_with_correct_args() {
        let mut mock_repo = MockCollectionRepository::new();
        let expected_query = CollectionQuery {
            page: 1,
            page_size: 10,
            sort_by: CollectionSortField::SetCode,
            sort_dir: SortDirection::Asc,
        };
        let expected_result = PaginatedCollection {
            items: vec![],
            total: 0,
            page: 1,
            page_size: 10,
        };
        let result_clone = expected_result.clone();

        mock_repo
            .expect_get_paginated()
            .withf(|uid, q| {
                uid == "user-1"
                    && q.page == 1
                    && q.page_size == 10
                    && q.sort_by == CollectionSortField::SetCode
                    && q.sort_dir == SortDirection::Asc
            })
            .returning(move |_, _| {
                let r = result_clone.clone();
                Box::pin(async move { Ok(r) })
            });

        let service = CollectionService::new(Arc::new(mock_repo));
        let result = service.get_collection("user-1", expected_query).await;
        assert!(result.is_ok());
        let paginated = result.unwrap();
        assert_eq!(paginated.page, 1);
        assert_eq!(paginated.page_size, 10);
        assert_eq!(paginated.total, 0);
    }

    #[tokio::test]
    async fn get_collection_propagates_repository_error() {
        let mut mock_repo = MockCollectionRepository::new();
        mock_repo.expect_get_paginated().returning(|_, _| {
            Box::pin(async { Err(AppError::RepositoryError("db error".to_string())) })
        });

        let service = CollectionService::new(Arc::new(mock_repo));
        let result = service
            .get_collection("user-1", CollectionQuery::default())
            .await;
        assert!(result.is_err());
    }
}
