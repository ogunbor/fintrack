use sqlx::MySqlPool;
use crate::{
    domain::DomainError,
    models::SignUpRequest,
    repositories::UserRepository,
};

pub struct AuthService;

impl AuthService {
    /// Validate if email is available for signup
    pub async fn validate_email_available(
        pool: &MySqlPool,
        email: &str,
    ) -> Result<(), DomainError> {
        let exists = UserRepository::exists_by_email(pool, email)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        if exists {
            return Err(DomainError::EmailAlreadyExists);
        }

        Ok(())
    }
}