use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/categories")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Categories: List")
}

#[post("/categories")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Categories: Create")
}

#[get("/categories/{id}")]
pub async fn show() -> impl Responder {
    HttpResponse::Ok().body("Categories: Show")
}

#[put("/categories/{id}")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Categories: Update")
}

#[delete("/categories/{id}")]
pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("Categories: Destroy")
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index)
       .service(create)
       .service(show)
       .service(update)
       .service(destroy);
}