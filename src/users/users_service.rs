use crate::{
    users::dto::{create_users_dto::CreateUserDTO, get_users_dto::GetUserDTO},
    utils::{
        errors::AppError,
        password::hash_password,
        query_builder::QueryBuilder,
        query_paginaton::QueryPagination,
        response_data::{ResponseData, ResponseDatas},
    },
};
use sqlx::PgPool;
use validator::Validate;

pub async fn create(
    pool: &PgPool,
    mut payload: CreateUserDTO,
) -> Result<ResponseData<GetUserDTO>, AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    payload.password = hash_password(payload.password).await?;

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

    let create_response = ResponseData::new(result);
    Ok(create_response)
}

pub async fn find_all(
    pool: &PgPool,
    pagination: QueryPagination,
) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError> {
    let QueryPagination { limit, page, order } = pagination;

    let (limit, offset, page, order, limit_str) =
        QueryPagination::paginate(&QueryPagination { limit, page, order });

    let count_query = QueryBuilder::new().from("users", "COUNT(*)").build();
    let count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    let query = QueryBuilder::new()
        .from("users", "*")
        .order_by("created_at", &order)
        .limit(limit)
        .offset(offset)
        .build();

    let result = sqlx::query_as::<_, GetUserDTO>(&query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(ResponseDatas::new(
        limit_str,
        page,
        count as u64,
        result.len(),
        order,
        result,
    ))
}
