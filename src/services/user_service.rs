use sqlx::MySqlPool;
use crate::{
    domain::{DomainError, User},
    models::UpdateProfileRequest,
    repositories::UserRepository,
};

pub struct UserService;

impl UserService {
    pub async fn get_by_id(pool: &MySqlPool, user_id: u64) -> Result<User, DomainError> {
        UserRepository::find_by_id(pool, user_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)
    }

    pub async fn update_profile(
        pool: &MySqlPool,
        user_id: u64,
        request: UpdateProfileRequest,
    ) -> Result<User, DomainError> {
        // Update profile
        UserRepository::update_profile(pool, user_id, &request.firstname, &request.lastname)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // Fetch updated user
        Self::get_by_id(pool, user_id).await
    }
}