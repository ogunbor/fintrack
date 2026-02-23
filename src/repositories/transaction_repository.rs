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
}