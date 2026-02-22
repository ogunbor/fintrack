use actix_web::{post, HttpResponse, Responder};

#[post("/auth/sign-up")]
pub async fn sign_up() -> impl Responder {
    HttpResponse::Ok().body("Sign Up")
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("Sign In")
}

// Configuration function to register routes
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(sign_up)
       .service(sign_in);
}