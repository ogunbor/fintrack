use actix_web::{get, post, HttpResponse, Responder};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("Profile")
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    HttpResponse::Ok().body("Update Profile")
}

// Configuration function to register routes
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_profile)
       .service(update_profile);
}