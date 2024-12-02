use crate::{server::AppState, users::dto::CreateUserDTO, utils::errors::AppError};
use actix_web::{guard, web, HttpResponse};
use sqlx::PgPool;

use super::{auth_service, dto::LoginDto};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .guard(guard::Post())
                    .route(web::post().to(register)),
            )
            .service(
                web::resource("/login")
                    .guard(guard::Post())
                    .route(web::post().to(login)),
            ),
    );
}

async fn register(
    pool: web::Data<PgPool>,
    app_state: web::Data<AppState>,
    payload: web::Json<CreateUserDTO>,
) -> Result<HttpResponse, AppError> {
    match auth_service::register(&pool, &app_state, payload.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(err) => Err(err),
    }
}

async fn login(
    pool: web::Data<PgPool>,
    app_state: web::Data<AppState>,
    payload: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    match auth_service::login(&pool, &app_state, payload.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}
