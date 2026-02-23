use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use crate::{
    api::{middleware::auth::get_user_id, AppState},
    models::CreateCategoryRequest,
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
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateCategoryRequest>,
) -> impl Responder {
    let user_id = get_user_id(&req);

    match CategoryService::create(&state.pool, user_id, data.into_inner()).await {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

#[get("/categories/{id}")]
pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<u64>,
) -> impl Responder {
    let user_id = get_user_id(&req);

    match CategoryService::get_by_id(&state.pool, id.into_inner(), user_id).await {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(e) => {
            use crate::domain::DomainError;
            match e {
                DomainError::NotFound => {
                    HttpResponse::NotFound().json(serde_json::json!({
                        "status": "error",
                        "message": e.to_string()
                    }))
                }
                DomainError::Unauthorized => {
                    HttpResponse::Forbidden().json(serde_json::json!({
                        "status": "error",
                        "message": e.to_string()
                    }))
                }
                _ => {
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status": "error",
                        "message": e.to_string()
                    }))
                }
            }
        }
    }
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