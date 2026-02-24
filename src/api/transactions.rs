use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use crate::{
    api::{middleware::auth::get_user_id, AppState},
    models::{CreateTransactionRequest, UpdateTransactionRequest},
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
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateTransactionRequest>,
) -> impl Responder {
    let user_id = get_user_id(&req);

    match TransactionService::create(&state.pool, user_id, data.into_inner()).await {
        Ok(transaction) => HttpResponse::Created().json(transaction),
        Err(e) => {
            use crate::domain::DomainError;
            match e {
                DomainError::NotFound => {
                    HttpResponse::NotFound().json(serde_json::json!({
                        "status": "error",
                        "message": "Category not found"
                    }))
                }
                DomainError::Unauthorized => {
                    HttpResponse::Forbidden().json(serde_json::json!({
                        "status": "error",
                        "message": "Unauthorized - category does not belong to you"
                    }))
                }
                DomainError::InsufficientBalance => {
                    HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "message": "Insufficient balance"
                    }))
                }
                DomainError::InvalidInput(msg) => {
                    HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "message": msg
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

#[get("/transactions/{id}")]
pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<u64>,
) -> impl Responder {
    let user_id = get_user_id(&req);

    match TransactionService::get_by_id(&state.pool, id.into_inner(), user_id).await {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => {
            use crate::domain::DomainError;
            match e {
                DomainError::NotFound => {
                    HttpResponse::NotFound().json(serde_json::json!({
                        "status": "error",
                        "message": "Transaction not found"
                    }))
                }
                DomainError::Unauthorized => {
                    HttpResponse::Forbidden().json(serde_json::json!({
                        "status": "error",
                        "message": "Unauthorized"
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

#[put("/transactions/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<u64>,
    data: web::Json<UpdateTransactionRequest>,
) -> impl Responder {
    let user_id = get_user_id(&req);

    match TransactionService::update(&state.pool, id.into_inner(), user_id, data.into_inner()).await {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => {
            use crate::domain::DomainError;
            match e {
                DomainError::NotFound => {
                    HttpResponse::NotFound().json(serde_json::json!({
                        "status": "error",
                        "message": "Transaction not found"
                    }))
                }
                DomainError::Unauthorized => {
                    HttpResponse::Forbidden().json(serde_json::json!({
                        "status": "error",
                        "message": "Unauthorized"
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