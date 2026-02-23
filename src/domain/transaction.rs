use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub r#type: String,  // "CREDIT" or "DEBIT"
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}