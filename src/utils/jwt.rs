use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: u64,        // Subject (user ID)
    pub role: String,    // User role
    pub exp: u64,        // Expiration time
}

pub fn create_jwt_token(user_id: u64, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 4 * 60 * 60; // 4 hours

    let claims = Claims {
        sub: user_id,
        role: "user".to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}