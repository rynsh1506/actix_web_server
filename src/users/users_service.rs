use super::dto::{
    create_users_dto::CreateUserDTO, get_users_dto::GetUserDTO, update_users_dto::UpdateUserDTO,
};
use crate::{
    users::users_query,
    utils::{
        errors::AppError,
        password::hash_password,
        query_paginaton::QueryPagination,
        response_data::{ResponseData, ResponseDatas},
    },
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub async fn create(
    pool: &PgPool,
    mut payload: CreateUserDTO,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    payload.validate().map_err(AppError::ValidationError)?;
    payload.password = hash_password(payload.password).await?;
    let result = users_query::create_user_query(pool, payload).await?;
    Ok(result)
}

pub async fn find_all(
    pool: &PgPool,
    pagination: QueryPagination,
) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError> {
    let result = users_query::find_all_users_query(pool, pagination).await?;
    Ok(result)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateUserDTO,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    payload.validate().map_err(AppError::ValidationError)?;
    if !users_query::check_existence(pool, id).await? {
        return Err(AppError::NotFound(format!("User with ID {} not found", id)));
    }
    let result = users_query::update_user_query(pool, id, payload).await?;
    Ok(result)
}

pub async fn find(pool: &PgPool, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError> {
    if !users_query::check_existence(pool, id).await? {
        return Err(AppError::NotFound(format!("User with ID {} not found", id)));
    }
    let result = users_query::find_user_query(pool, id).await?;
    Ok(result)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError> {
    if !users_query::check_existence(pool, id).await? {
        return Err(AppError::NotFound(format!("User with ID {} not found", id)));
    }
    let result = users_query::delete_user_query(pool, id).await?;
    Ok(result)
}
