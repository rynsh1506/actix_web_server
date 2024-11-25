use crate::{
    users::{
        dto::{create_users_dto::CreateUserDTO, update_users_dto::UpdateUserDTO},
        users_service,
    },
    utils::{errors::AppError, query_paginaton::QueryPagination},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;

pub fn handler() -> actix_web::Scope {
    web::scope("/users")
        .service(find)
        .service(find_all)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("/{id}")]
async fn find() -> HttpResponse {
    HttpResponse::Ok().body("test")
}

#[get("")]
async fn find_all(
    pool: web::Data<PgPool>,
    pagination: web::Query<QueryPagination>,
) -> Result<HttpResponse, AppError> {
    match users_service::find_all(&pool, pagination).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

#[post("")]
async fn create(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUserDTO>,
) -> Result<HttpResponse, AppError> {
    match users_service::create(&pool, payload).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(err) => Err(err),
    }
}

#[put("/{id}")]
async fn update(_payload: web::Json<UpdateUserDTO>) -> HttpResponse {
    HttpResponse::Ok().body("test")
}

#[delete("")]
async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("test")
}
