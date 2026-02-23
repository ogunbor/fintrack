use crate::{
    domain::{Category, DomainError},
    models::{CreateCategoryRequest, UpdateCategoryRequest},
    repositories::CategoryRepository,
};
use sqlx::MySqlPool;

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
        let category_id = CategoryRepository::create(
            pool,
            user_id,
            &request.name,
            request.description.as_deref(),
        )
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        CategoryRepository::find_by_id(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)
    }

    /// Get category by ID (with ownership check)
    pub async fn get_by_id(
        pool: &MySqlPool,
        category_id: u64,
        user_id: u64,
    ) -> Result<Category, DomainError> {
        let category = CategoryRepository::find_by_id(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)?;

        // Verify ownership
        if category.user_id != user_id {
            return Err(DomainError::Unauthorized);
        }

        Ok(category)
    }

    pub async fn update(
        pool: &MySqlPool,
        category_id: u64,
        user_id: u64,
        request: UpdateCategoryRequest,
    ) -> Result<Category, DomainError> {
        // 1. Fetch category
        let category = CategoryRepository::find_by_id(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)?;

        // 2. Verify ownership
        if category.user_id != user_id {
            return Err(DomainError::Unauthorized);
        }

        // 3. Update category
        CategoryRepository::update(
            pool,
            category_id,
            &request.name,
            request.description.as_deref(),
        )
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // 4. Fetch and return updated category
        CategoryRepository::find_by_id(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)
    }

    /// Delete a category (with ownership verification)
    pub async fn delete(
        pool: &MySqlPool,
        category_id: u64,
        user_id: u64,
    ) -> Result<(), DomainError> {
        // 1. Fetch category
        let category = CategoryRepository::find_by_id(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)?;

        // 2. Verify ownership
        if category.user_id != user_id {
            return Err(DomainError::Unauthorized);
        }

        // 3. Delete category
        CategoryRepository::delete(pool, category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
