use sqlx::MySqlPool;
use crate::{
    domain::{DomainError, Transaction},
    models::CreateTransactionRequest,
    repositories::{CategoryRepository, TransactionRepository, UserRepository},
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

    /// Create a new transaction with balance updates
    pub async fn create(
        pool: &MySqlPool,
        user_id: u64,
        request: CreateTransactionRequest,
    ) -> Result<Transaction, DomainError> {
        // 1. Fetch user
        let user = UserRepository::find_by_id(pool, user_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)?;

        // 2. Fetch category and verify ownership
        let category = CategoryRepository::find_by_id(pool, request.category_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)?;

        if category.user_id != user_id {
            return Err(DomainError::Unauthorized);
        }

        // 3. Validate transaction type
        if request.r#type != "CREDIT" && request.r#type != "DEBIT" {
            return Err(DomainError::InvalidInput("Transaction type must be CREDIT or DEBIT".to_string()));
        }

        // 4. Check balance for DEBIT transactions
        if request.r#type == "DEBIT" {
            if user.balance < request.amount {
                return Err(DomainError::InsufficientBalance);
            }
            if category.balance < request.amount {
                return Err(DomainError::InsufficientBalance);
            }
        }

        // 5. Calculate new balances
        let new_user_balance = if request.r#type == "DEBIT" {
            user.balance - request.amount
        } else {
            user.balance + request.amount
        };

        let new_category_balance = if request.r#type == "DEBIT" {
            category.balance - request.amount
        } else {
            category.balance + request.amount
        };

        // 6. Create transaction
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

        // 7. Update user balance
        UserRepository::update_balance(pool, user_id, new_user_balance)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // 8. Update category balance
        CategoryRepository::update_balance(pool, category.id, new_category_balance)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // 9. Fetch and return created transaction
        TransactionRepository::find_by_id(pool, transaction_id)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or(DomainError::NotFound)
    }
}