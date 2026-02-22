use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use crate::{
    api::AppState,
    models::SignUpRequest,
    services::AuthService,
};

#[post("/auth/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> impl Responder {
    // Validate email availability
    match AuthService::validate_email_available(&state.pool, &data.email).await {
        Ok(_) => {
            // Email is available
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": format!("Sign Up: {:?}", data)
            }))
        }
        Err(e) => {
            // Email already exists or database error
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("Sign In")
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(sign_up)
       .service(sign_in);
}