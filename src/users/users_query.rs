use crate::{
    auth::dto::login_dto::GetLoginDto,
    users::{
        dto::{CreateUserDTO, GetUserDTO, UpdateUserDTO},
        entity::User,
    },
    utils::{
        errors::AppError,
        query_paginaton::{QueryPagination, ResultWithPagination},
    },
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn login_users_query(pool: &PgPool, email: &str) -> Result<GetLoginDto, AppError> {
    let result: GetLoginDto = sqlx::query_as!(
        GetLoginDto,
        "--sql
        SELECT
            id, password, email
        FROM 
            users
        WHERE email = $1 
        ",
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!(
        "User with email {} not found",
        email
    )))?;

    Ok(result)
}

pub async fn delete_user_query(pool: &PgPool, id: Uuid) -> Result<GetUserDTO, AppError> {
    let result: GetUserDTO = sqlx::query_as!(
        User,
        "--sql
        DELETE FROM users
        WHERE
            id = $1
        RETURNING
            *
        ",
        id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}

pub async fn find_user_query(pool: &PgPool, id: Uuid) -> Result<GetUserDTO, AppError> {
    let result: GetUserDTO = sqlx::query_as!(
        User,
        "--sql
        SELECT
        *
        FROM
            users
        WHERE
            id = $1
        ",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}

pub async fn update_user_query(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateUserDTO,
) -> Result<GetUserDTO, AppError> {
    let User {
        name,
        email,
        updated_at,
        ..
    } = payload.into();

    let result: GetUserDTO = sqlx::query_as!(
        User,
        "--sql
        UPDATE
            users
        SET 
            name = $1, 
            email = $2,
            updated_at = $3
        WHERE 
            id = $4
        RETURNING *
        ",
        name,
        email,
        updated_at,
        id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}

pub async fn create_user_query(pool: &PgPool, payload: CreateUserDTO) -> Result<Uuid, AppError> {
    let User {
        id,
        name,
        email,
        password,
        created_at,
        updated_at,
    } = payload.into();

    let user_id: Uuid = sqlx::query_scalar!(
        "--sql
        INSERT INTO
            users (id, name, email, password, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, $5, $6)
        RETURNING id
        ",
        id,
        name,
        email,
        password,
        created_at,
        updated_at,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(err) if err.is_unique_violation() => match err.constraint() {
            Some(constraint) => AppError::Conflict(format!("{} already exists.", constraint)),
            None => AppError::Conflict("Unique constraint violation.".to_string()),
        },
        _ => AppError::DatabaseError(e),
    })?;

    Ok(user_id)
}

pub async fn find_all_users_query(
    pool: &PgPool,
    query_pagination: QueryPagination,
) -> Result<ResultWithPagination<Vec<GetUserDTO>>, AppError> {
    let (limit, offset, page, order) = query_pagination.paginate();

    let count: i64 = sqlx::query_scalar!(
        "--sql
        SELECT
            COUNT(*)
        FROM
            users
        "
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .unwrap_or_default();

    let mut query = String::new();
    for (key, value) in order.iter() {
        query = format!(
            "--sql
            SELECT 
                *
            FROM
                users
            ORDER BY {} {}
            LIMIT $1 OFFSET $2
            ",
            key, value
        );
    }

    let result: Vec<GetUserDTO> = sqlx::query_as::<_, User>(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .into_iter()
        .map(|user| user.into())
        .collect();

    Ok(ResultWithPagination::new(
        limit,
        page,
        count,
        result.len(),
        result,
    ))
}
