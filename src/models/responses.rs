use crate::domain::Category;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub status: String,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            user_id: category.user_id,
            name: category.name,
            description: category.description,
            balance: category.balance,
            created_at: category.created_at.to_string(),
            updated_at: category.updated_at.to_string(),
        }
    }
}
