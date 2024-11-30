use super::dto::{create_users_dto::CreateUserDTO, update_users_dto::UpdateUserDTO};
use crate::{
    users::users_service,
    utils::{errors::AppError, query_paginaton::QueryPagination},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_qs::actix::QsQuery;
use sqlx::PgPool;
use uuid::Uuid;

pub fn handler() -> actix_web::Scope {
    web::scope("/users")
        .service(find)
        .service(find_all)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("/{id}")]
async fn find(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    match users_service::find(&pool, id.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

#[get("")]
async fn find_all(
    pool: web::Data<PgPool>,
    pagination: QsQuery<QueryPagination>,
) -> Result<HttpResponse, AppError> {
    match users_service::find_all(&pool, pagination.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

#[post("")]
async fn create(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUserDTO>,
) -> Result<HttpResponse, AppError> {
    match users_service::create(&pool, payload.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(err) => Err(err),
    }
}

#[put("/{id}")]
async fn update(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateUserDTO>,
) -> Result<HttpResponse, AppError> {
    match users_service::update(&pool, id.into_inner(), payload.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

#[delete("/{id}")]
async fn delete(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    match users_service::delete(&pool, id.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}
