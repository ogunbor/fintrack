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