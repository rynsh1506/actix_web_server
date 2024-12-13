use crate::{
    users::{
        dto::{GetUserDTO, UpdateUserDTO},
        users_query,
    },
    utils::{
        auth::validate_user_id_in_token,
        errors::AppError,
        query_paginaton::QueryPagination,
        response_data::{ResponseData, ResponseDatas},
    },
};
use actix_web::HttpRequest;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub async fn find(pool: &PgPool, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError> {
    let result = users_query::find_user(pool, id).await?;
    Ok(ResponseData::new(
        result,
        "Data has been successfuly retrieved.",
    ))
}

pub async fn find_all(
    pool: &PgPool,
    query_pagination: QueryPagination,
) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError> {
    let result = users_query::find_all_user(pool, query_pagination).await?;

    Ok(ResponseDatas::new(
        result.limit,
        result.page,
        result.count,
        result.current_count,
        result.data,
    ))
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateUserDTO,
    req: &HttpRequest,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    validate_user_id_in_token(req, &id)?;

    payload.validate().map_err(AppError::ValidationError)?;

    let result = users_query::update_user(pool, id, payload).await?;

    Ok(ResponseData::new(
        result,
        "Data has been successfuly updated.",
    ))
}

pub async fn delete(
    pool: &PgPool,
    id: Uuid,
    req: &HttpRequest,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    validate_user_id_in_token(req, &id)?;

    let result = users_query::delete_user(pool, id).await?;
    Ok(ResponseData::new(
        result,
        "Data has been successfuly deleted.",
    ))
}

pub async fn soft_delete(
    pool: &PgPool,
    id: Uuid,
    req: &HttpRequest,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    validate_user_id_in_token(req, &id)?;

    let result = users_query::delete_user_with_status(pool, id).await?;
    Ok(ResponseData::new(
        result,
        "Data has been successfuly deleted.",
    ))
}
