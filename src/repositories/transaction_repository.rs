use sqlx::MySqlPool;
use crate::domain::Transaction;

pub struct TransactionRepository;

impl TransactionRepository {
    /// Get all transactions for a user
    pub async fn find_all_by_user(
        pool: &MySqlPool,
        user_id: u64,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, user_id, category_id, type, amount, memo, description, created_at, updated_at FROM transactions WHERE user_id = ?",
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| Transaction {
            id: r.id,
            user_id: r.user_id,
            category_id: r.category_id,
            r#type: r.r#type,
            amount: r.amount,
            memo: r.memo,
            description: r.description,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect())
    }

    /// Find transaction by ID
    pub async fn find_by_id(
        pool: &MySqlPool,
        id: u64,
    ) -> Result<Option<Transaction>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, user_id, category_id, type, amount, memo, description, created_at, updated_at FROM transactions WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Transaction {
            id: r.id,
            user_id: r.user_id,
            category_id: r.category_id,
            r#type: r.r#type,
            amount: r.amount,
            memo: r.memo,
            description: r.description,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    /// Create a new transaction
    pub async fn create(
        pool: &MySqlPool,
        user_id: u64,
        category_id: u64,
        r#type: &str,
        amount: u64,
        memo: &str,
        description: Option<&str>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO transactions (user_id, category_id, type, amount, memo, description) VALUES (?, ?, ?, ?, ?, ?)",
            user_id,
            category_id,
            r#type,
            amount,
            memo,
            description
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id())
    }
     /// Update a transaction (memo and description only)
    pub async fn update(
        pool: &MySqlPool,
        id: u64,
        memo: &str,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE transactions SET memo = ?, description = ? WHERE id = ?",
            memo,
            description,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
     /// Delete a transaction
    pub async fn delete(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM transactions WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(())
    }
}