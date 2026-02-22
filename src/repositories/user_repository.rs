use sqlx::MySqlPool;

pub struct UserRepository;

impl UserRepository {
    /// Check if a user with the given email exists
    pub async fn exists_by_email(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT id FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        
        Ok(result.is_some())
    }

    /// Create a new user
    pub async fn create(
        pool: &MySqlPool,
        email: &str,
        password_hash: &str,
        firstname: &str,
        lastname: &str,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO users (email, password, firstname, lastname) VALUES (?, ?, ?, ?)"
        )
        .bind(email)
        .bind(password_hash)
        .bind(firstname)
        .bind(lastname)
        .execute(pool)
        .await?;

        Ok(result.last_insert_id())
    }
}