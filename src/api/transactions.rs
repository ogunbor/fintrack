use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use crate::{
    api::{middleware::auth::get_user_id, AppState},
    services::TransactionService,
};

#[get("/transactions")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = get_user_id(&req);

    match TransactionService::get_all_for_user(&state.pool, user_id).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
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