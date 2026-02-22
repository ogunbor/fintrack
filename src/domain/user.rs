use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub firstname: String,
    pub lastname: String,
    pub balance: u64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}