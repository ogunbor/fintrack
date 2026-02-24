use actix_web::{post, web, HttpResponse, Responder};
use crate::{
    api::AppState,
    models::{SignInRequest, SignUpRequest},
    services::AuthService,
};

#[post("/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> impl Responder {
    match AuthService::sign_up(&state.pool, data.into_inner()).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "User created successfully"
        })),
        Err(e) => {
            use crate::domain::DomainError;
            match e {
                DomainError::EmailAlreadyExists => {
                    HttpResponse::Conflict().json(serde_json::json!({
                        "status": "error",
                        "message": "Email already exists"
                    }))
                }
                _ => {
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "error",
                        "message": e.to_string()
                    }))
                }
            }
        }
    }
}

#[post("/sign-in")]
pub async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInRequest>,
) -> impl Responder {
    match AuthService::sign_in(&state.pool, data.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "token": token
        })),
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "error",
            "message": "Invalid credentials"
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up)
       .service(sign_in);
}