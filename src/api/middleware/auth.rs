use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
    web, Error, HttpMessage,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use crate::{api::AppState, utils::jwt::Claims};

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {
    // 1. Get Authorization header
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is missing"
        }))
    })?;

    // 2. Parse header value
    let auth_str = auth_header.to_str().map_err(|_| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is malformed"
        }))
    })?;

    // 3. Check Bearer prefix
    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header must start with 'Bearer '"
        })));
    }

    // 4. Extract token
    let token = auth_str.strip_prefix("Bearer ").unwrap();

    // 5. Get JWT secret from app state
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());

    // 6. Verify and decode token
    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            // Store user ID in request extensions for handlers to access
            req.extensions_mut().insert(token_data.claims.sub);
            next.call(req).await
        }
        Err(_) => Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid or expired token"
        }))),
    }
}

/// Helper function to extract user_id from request extensions
pub fn get_user_id(req: &actix_web::HttpRequest) -> u64 {
    let ext = req.extensions();
    *ext.get::<u64>()
        .expect("User ID not found in request extensions")
}
