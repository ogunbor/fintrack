use crate::{
    api::AppState,
    models::{AuthResponse, MessageResponse, SignInRequest, SignUpRequest},
    services::AuthService,
    utils::jwt::create_jwt_token,
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    match AuthService::sign_up(&state.pool, data.into_inner()).await {
        Ok(_user_id) => HttpResponse::Created().json(MessageResponse {
            status: "success".to_string(),
            message: "Account created successfully".to_string(),
        }),
        Err(e) => HttpResponse::UnprocessableEntity().json(MessageResponse {
            status: "error".to_string(),
            message: e.to_string(),
        }),
    }
}

#[post("/auth/sign-in")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> impl Responder {
    match AuthService::sign_in(&state.pool, data.into_inner()).await {
        Ok(user) => {
            // Generate JWT token
            match create_jwt_token(user.id, &state.jwt_secret) {
                Ok(token) => HttpResponse::Ok().json(AuthResponse {
                    status: "success".to_string(),
                    token,
                }),
                Err(_) => HttpResponse::InternalServerError().json(MessageResponse {
                    status: "error".to_string(),
                    message: "Failed to generate token".to_string(),
                }),
            }
        }
        Err(e) => HttpResponse::Unauthorized().json(MessageResponse {
            status: "error".to_string(),
            message: e.to_string(),
        }),
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(sign_up).service(sign_in);
}
