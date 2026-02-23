use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/transactions")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Transactions: List")
}

#[post("/transactions")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Transactions: Create")
}

#[get("/transactions/{id}")]
pub async fn show() -> impl Responder {
    HttpResponse::Ok().body("Transactions: Show")
}

#[put("/transactions/{id}")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Transactions: Update")
}

#[delete("/transactions/{id}")]
pub async fn destroy() -> impl Responder {
    HttpResponse::Ok().body("Transactions: Destroy")
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index)
       .service(create)
       .service(show)
       .service(update)
       .service(destroy);
}