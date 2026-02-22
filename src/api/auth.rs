use actix_web::{post, web, HttpResponse, Responder};
use crate::models::SignUpRequest;

#[post("/auth/sign-up")]
pub async fn sign_up(data: web::Json<SignUpRequest>) -> impl Responder {
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