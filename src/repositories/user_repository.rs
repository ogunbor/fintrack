use crate::domain::User;
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
            "INSERT INTO users (email, password, firstname, lastname) VALUES (?, ?, ?, ?)",
        )
        .bind(email)
        .bind(password_hash)
        .bind(firstname)
        .bind(lastname)
        .execute(pool)
        .await?;

        Ok(result.last_insert_id())
    }

    /// Find user by email (for sign-in)
    pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, email, password, firstname, lastname, balance, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password,
            firstname: r.firstname,
            lastname: r.lastname,
            balance: r.balance,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, email, password, firstname, lastname, balance, created_at, updated_at FROM users WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password,
            firstname: r.firstname,
            lastname: r.lastname,
            balance: r.balance,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    /// Update user profile
    pub async fn update_profile(
        pool: &MySqlPool,
        id: u64,
        firstname: &str,
        lastname: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET firstname = ?, lastname = ? WHERE id = ?",
            firstname,
            lastname,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
