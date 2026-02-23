use sqlx::MySqlPool;
use crate::domain::Category;

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

        Ok(rows.into_iter().map(|r| Category {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            description: r.description,
            balance: r.balance,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect())
    }
}