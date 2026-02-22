use actix_web::{post, web, HttpResponse, Responder};
use crate::models::SignUpRequest;
use crate::api::AppState;

#[post("/auth/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> impl Responder {
    // Access the pool directly - no lock needed!
    let _pool = &state.pool;
    
    HttpResponse::Ok().body(format!("Sign Up: {:?}", data))
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("Sign In")
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(sign_up)
       .service(sign_in);
}