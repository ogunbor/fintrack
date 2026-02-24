use sqlx::MySqlPool;
use crate::{
    domain::{DomainError, Transaction},
    models::CreateTransactionRequest,
    repositories::{CategoryRepository, TransactionRepository},
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

    /// Create a new transaction
    pub async fn create(
        pool: &MySqlPool,
        user_id: u64,
        request: CreateTransactionRequest,
    ) -> Result<Transaction, DomainError> {
        // 1. Fetch category and verify ownership
        let category = CategoryRepository::find_by_id(pool, request.category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)?;

        if category.user_id != user_id {
            return Err(DomainError::Unauthorized);
        }

        // 2. Validate transaction type
        if request.r#type != "CREDIT" && request.r#type != "DEBIT" {
            return Err(DomainError::InvalidInput("Transaction type must be CREDIT or DEBIT".to_string()));
        }

        // 3. Create transaction
        let transaction_id = TransactionRepository::create(
            pool,
            user_id,
            request.category_id,
            &request.r#type,
            request.amount,
            &request.memo,
            request.description.as_deref(),
        )
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // 4. Fetch and return created transaction
        TransactionRepository::find_by_id(pool, transaction_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)
    }
}