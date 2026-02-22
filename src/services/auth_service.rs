use sqlx::MySqlPool;
use crate::{
    domain::DomainError,
    models::SignUpRequest,
    repositories::UserRepository,
    utils::password::hash_password,
};

pub struct AuthService;

impl AuthService {
    /// Sign up a new user
    pub async fn sign_up(
        pool: &MySqlPool,
        request: SignUpRequest,
    ) -> Result<u64, DomainError> {
        // 1. Check if email already exists
        let email_exists = UserRepository::exists_by_email(pool, &request.email)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        if email_exists {
            return Err(DomainError::EmailAlreadyExists);
        }

        // 2. Hash password
        let password_hash = hash_password(&request.password)
            .map_err(|_| DomainError::DatabaseError("Password hashing failed".to_string()))?;

        // 3. Create user
        let user_id = UserRepository::create(
            pool,
            &request.email,
            &password_hash,
            &request.firstname,
            &request.lastname,
        )
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(user_id)
    }
}