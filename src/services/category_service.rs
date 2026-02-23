use sqlx::MySqlPool;
use crate::{
    domain::{Category, DomainError},
    models::CreateCategoryRequest,
    repositories::CategoryRepository,
};

pub struct CategoryService;

impl CategoryService {
    /// Get all categories for a user
    pub async fn get_all_for_user(
        pool: &MySqlPool,
        user_id: u64,
    ) -> Result<Vec<Category>, DomainError> {
        CategoryRepository::find_all_by_user(pool, user_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))
    }

    /// Create a new category
    pub async fn create(
        pool: &MySqlPool,
        user_id: u64,
        request: CreateCategoryRequest,
    ) -> Result<Category, DomainError> {
        // Create category
        let category_id = CategoryRepository::create(
            pool,
            user_id,
            &request.name,
            request.description.as_deref(),
        )
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // Fetch and return created category
        CategoryRepository::find_by_id(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)
    }
}