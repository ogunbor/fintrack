use crate::domain::Category;
use sqlx::MySqlPool;

pub struct CategoryRepository;

impl CategoryRepository {
    /// Get all categories for a user
    pub async fn find_all_by_user(
        pool: &MySqlPool,
        user_id: u64,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, user_id, name, description, balance, created_at, updated_at FROM categories WHERE user_id = ?",
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Category {
                id: r.id,
                user_id: r.user_id,
                name: r.name,
                description: r.description,
                balance: r.balance,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    /// Find category by ID
    pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Category>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, user_id, name, description, balance, created_at, updated_at FROM categories WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Category {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            description: r.description,
            balance: r.balance,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    /// Create a new category
    pub async fn create(
        pool: &MySqlPool,
        user_id: u64,
        name: &str,
        description: Option<&str>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO categories (user_id, name, description) VALUES (?, ?, ?)",
            user_id,
            name,
            description
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id())
    }

    pub async fn update(
        pool: &MySqlPool,
        id: u64,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE categories SET name = ?, description = ? WHERE id = ?",
            name,
            description,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
