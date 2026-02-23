use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use crate::{
    api::{middleware::auth::get_user_id, AppState},
    services::CategoryService,
};

#[get("/categories")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = get_user_id(&req);

    match CategoryService::get_all_for_user(&state.pool, user_id).await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
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
