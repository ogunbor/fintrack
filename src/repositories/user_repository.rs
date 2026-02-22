use sqlx::MySqlPool;

pub struct UserRepository;

impl UserRepository {
    /// Check if a user with the given email exists
    pub async fn exists_by_email(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("SELECT id FROM users WHERE email = ?", email)
            .fetch_optional(pool)
            .await?;
        
        Ok(result.is_some())
    }
}