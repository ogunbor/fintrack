use sqlx::MySqlPool;
use crate::{
    domain::{Category, DomainError},
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
}