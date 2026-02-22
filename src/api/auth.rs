use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use crate::{
    api::AppState,
    models::{MessageResponse, SignUpRequest},
    services::AuthService,
};

#[post("/auth/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> impl Responder {
    match AuthService::sign_up(&state.pool, data.into_inner()).await {
        Ok(_user_id) => {
            HttpResponse::Created().json(MessageResponse {
                status: "success".to_string(),
                message: "Account created successfully".to_string(),
            })
        }
        Err(e) => {
            HttpResponse::UnprocessableEntity().json(MessageResponse {
                status: "error".to_string(),
                message: e.to_string(),
            })
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
