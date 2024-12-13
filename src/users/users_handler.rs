use std::collections::HashMap;

use crate::{
    middlewares::middleware_auth::JwtAuthMiddleware,
    server::AppState,
    users::{dto::UpdateUserDTO, users_service},
    utils::{errors::AppError, query_paginaton::QueryPagination},
};
use actix_web::{web, HttpRequest, HttpResponse};
use serde_qs::actix::QsQuery;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    cfg.service(
        web::scope("/users")
            .wrap(JwtAuthMiddleware::new(app_state))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(find))
                    .route(web::put().to(update))
                    .route(web::delete().to(delete)),
            )
            .service(web::resource("").route(web::get().to(find_all))),
    );
}

async fn find(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    match users_service::find(&pool, id.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

async fn find_all(
    pool: web::Data<PgPool>,
    query_pagination: QsQuery<QueryPagination>,
) -> Result<HttpResponse, AppError> {
    match users_service::find_all(&pool, query_pagination.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

async fn update(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateUserDTO>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    match users_service::update(&pool, id.into_inner(), payload.into_inner(), &req).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

async fn delete(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    query: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mode = query.get("mode").map(|s| s.as_str());

    if let Some("hard") = mode {
        match users_service::delete(&pool, id.into_inner(), &req).await {
            Ok(response) => return Ok(HttpResponse::Ok().json(response)),
            Err(err) => return Err(err),
        }
    }

    match users_service::soft_delete(&pool, id.into_inner(), &req).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}
