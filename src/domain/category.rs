use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}