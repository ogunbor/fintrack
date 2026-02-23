use sqlx::MySqlPool;
use crate::{
    domain::{DomainError, Transaction},
    repositories::TransactionRepository,
};

pub struct TransactionService;

impl TransactionService {
    /// Get all transactions for a user
    pub async fn get_all_for_user(
        pool: &MySqlPool,
        user_id: u64,
    ) -> Result<Vec<Transaction>, DomainError> {
        TransactionRepository::find_all_by_user(pool, user_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))
    }
}