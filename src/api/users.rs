use crate::{
    api::{middleware::auth::get_user_id, AppState},
    models::UpdateProfileRequest,
    services::UserService,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/me")]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = get_user_id(&req);

    match UserService::get_by_id(&state.pool, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

#[post("/me")]
pub async fn update_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<UpdateProfileRequest>,
) -> impl Responder {
    let user_id = get_user_id(&req);

    match UserService::update_profile(&state.pool, user_id, data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_profile).service(update_profile);
}
