use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub category_id: u64,
    pub r#type: String,  // "CREDIT" or "DEBIT"
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransactionRequest {
    pub memo: String,
    pub description: Option<String>,
}