use super::dto::{
    create_users_dto::CreateUserDTO, get_users_dto::GetUserDTO, update_users_dto::UpdateUserDTO,
};
use crate::utils::{
    errors::AppError,
    query_builder::QueryBuilder,
    query_paginaton::QueryPagination,
    response_data::{ResponseData, ResponseDatas},
};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

pub async fn check_existence(pool: &Pool<Postgres>, id: Uuid) -> Result<bool, AppError> {
    let query = QueryBuilder::new()
        .from("users", "COUNT(*)")
        .where_clause()
        .condition("id = $1")
        .build();
    let count: i64 = sqlx::query_scalar(&query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(count > 0)
}

pub async fn delete_user_query(
    pool: &PgPool,
    id: Uuid,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    let query = QueryBuilder::new()
        .delete("users")
        .where_clause()
        .condition("id = $1")
        .returning("id, name, email, created_at, updated_at")
        .build();

    let result = sqlx::query_as::<_, GetUserDTO>(&query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(ResponseData::new(result))
}

pub async fn find_user_query(
    pool: &PgPool,
    id: Uuid,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    let query = QueryBuilder::new()
        .from("users", "*")
        .where_clause()
        .condition("id = $1")
        .build();

    let result = sqlx::query_as::<_, GetUserDTO>(&query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(ResponseData::new(result))
}

pub async fn update_user_query(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateUserDTO,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    let UpdateUserDTO {
        name,
        email,
        updated_at,
        ..
    } = payload;

    let query = QueryBuilder::new()
        .update("users", "name = $1, email = $2, updated_at = $3")
        .where_clause()
        .condition("id = $4")
        .returning("id, name, email, created_at, updated_at")
        .build();

    let result = sqlx::query_as::<_, GetUserDTO>(&query)
        .bind(name)
        .bind(email)
        .bind(updated_at)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(ResponseData::new(result))
}

pub async fn create_user_query(
    pool: &PgPool,
    payload: CreateUserDTO,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    let query = QueryBuilder::new()
        .insert("users", "name, password, email", "$1, $2, $3")
        .returning("id, name, email, created_at, updated_at")
        .build();
    let result = sqlx::query_as::<_, GetUserDTO>(&query)
        .bind(payload.name)
        .bind(payload.password)
        .bind(payload.email)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(ResponseData::new(result))
}

pub async fn find_all_users_query(
    pool: &PgPool,
    pagination: QueryPagination,
) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError> {
    let (limit, offset, page, order) = pagination.paginate();
    let count_query = QueryBuilder::new().from("users", "COUNT(*)").build();
    let count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    let mut query = QueryBuilder::new().from("users", "*");
    for (key, value) in order.iter() {
        query = query.order_by(key, value);
    }
    let query = query.limit(limit).offset(offset).build();

    let result = sqlx::query_as::<_, GetUserDTO>(&query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(ResponseDatas::new(limit, page, count, result.len(), result))
}
