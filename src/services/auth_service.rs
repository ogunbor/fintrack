use sqlx::MySqlPool;
use crate::{
    domain::{DomainError, User},
    models::{SignInRequest, SignUpRequest},
    repositories::UserRepository,
    utils::password::{hash_password, verify_password},
};

pub struct AuthService;

impl AuthService {
    /// Sign up a new user
    pub async fn sign_up(
        pool: &MySqlPool,
        request: SignUpRequest,
    ) -> Result<u64, DomainError> {
        // Check if email already exists
        let email_exists = UserRepository::exists_by_email(pool, &request.email)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        if email_exists {
            return Err(DomainError::EmailAlreadyExists);
        }

        // Hash password
        let password_hash = hash_password(&request.password)
            .map_err(|_| DomainError::DatabaseError("Password hashing failed".to_string()))?;

        // Create user
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

    /// Sign in an existing user
    pub async fn sign_in(
        pool: &MySqlPool,
        request: SignInRequest,
    ) -> Result<User, DomainError> {
        // Find user by email
        let user = UserRepository::find_by_email(pool, &request.email)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::InvalidCredentials)?;

        // Verify password
        let password_valid = verify_password(&request.password, &user.password_hash)
            .map_err(|_| DomainError::DatabaseError("Password verification failed".to_string()))?;

        if !password_valid {
            return Err(DomainError::InvalidCredentials);
        }

        Ok(user)
    }
}