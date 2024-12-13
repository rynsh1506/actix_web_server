use crate::{
    auth::dto::login_dto::GetLoginDto,
    users::{
        dto::{CreateUserDTO, GetUserDTO, UpdateUserDTO, UpdateUserPasswordDto},
        entity::{User, UserStatus},
    },
    utils::{
        errors::AppError,
        query_paginaton::{QueryPagination, ResultWithPagination},
    },
};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

pub async fn login_users_query(pool: &PgPool, email: &str) -> Result<GetLoginDto, AppError> {
    let result: GetLoginDto = sqlx::query_as::<_, GetLoginDto>(
        "--sql
        SELECT
            id, password, email
        FROM 
            users
        WHERE 
            email = $1 AND status = 'ACTIVE'
        ",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!(
        "User with email {} not found",
        email
    )))?;

    Ok(result)
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<GetUserDTO, AppError> {
    let result: GetUserDTO = sqlx::query_as::<_, User>(
        r#"--sql
        DELETE FROM users
        WHERE
            id = $1 AND status = $2
        RETURNING 
            *
        "#,
    )
    .bind(id)
    .bind(UserStatus::DELETED)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}

pub async fn find_user(pool: &PgPool, id: Uuid) -> Result<GetUserDTO, AppError> {
    let result: GetUserDTO = sqlx::query_as::<_, User>(
        r#"--sql
        SELECT
        *
        FROM
            users
        WHERE
            id = $1 AND status = $2
        "#,
    )
    .bind(id)
    .bind(UserStatus::ACTIVE)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}

pub async fn update_user(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateUserDTO,
) -> Result<GetUserDTO, AppError> {
    enum DataType {
        Text(String),
        DateTime(Option<DateTime<Utc>>),
        UserStatus(UserStatus),
    }

    let input: User = payload.into();

    let mut query_builder = QueryBuilder::new("UPDATE users SET ");
    let mut updates: Vec<(&str, DataType)> = Vec::new();

    if let Some(name) = input.name {
        updates.push(("name", DataType::Text(name)));
    }

    if let Some(email) = input.email {
        updates.push(("email", DataType::Text(email)));
    }

    if let Some(status) = input.status {
        updates.push(("status", DataType::UserStatus(status)));
    }

    if let Some(updated_at) = input.updated_at {
        updates.push(("updated_at", DataType::DateTime(Some(updated_at))));
    }

    for (i, (col, val)) in updates.iter().enumerate() {
        if i > 0 {
            query_builder.push(", ");
        }
        query_builder.push(format!("{} = ", col));
        match val {
            DataType::Text(value) => query_builder.push_bind(value),
            DataType::DateTime(value) => query_builder.push_bind(value),
            DataType::UserStatus(value) => query_builder.push_bind(value),
        };
    }

    query_builder.push(" WHERE id = ").push_bind(id);
    query_builder
        .push(" AND status != ")
        .push_bind(UserStatus::DELETED);
    query_builder.push(" RETURNING *");

    let query = query_builder.build_query_as::<User>();

    let result: GetUserDTO = query
        .fetch_optional(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
        .into();

    Ok(result)
}

pub async fn create_user(pool: &PgPool, payload: CreateUserDTO) -> Result<Uuid, AppError> {
    let User {
        id,
        name,
        email,
        password,
        status,
        created_at,
        updated_at,
        deleted_at,
    } = payload.into();

    let user_id: Uuid = sqlx::query_scalar(
        r#"--sql
        INSERT INTO
            users (id, name, email, password, status, created_at, updated_at, deleted_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .bind(password)
    .bind(status)
    .bind(created_at)
    .bind(updated_at)
    .bind(deleted_at)
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

pub async fn find_all_user(
    pool: &PgPool,
    query_pagination: QueryPagination,
) -> Result<ResultWithPagination<Vec<GetUserDTO>>, AppError> {
    let (limit, offset, page, order) = query_pagination.paginate();

    let count: i64 = sqlx::query_scalar::<_, i64>(
        "--sql
        SELECT
            COUNT(*)
        FROM
            users
        ",
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let mut query = String::new();
    for (key, value) in order.iter() {
        query = format!(
            "--sql
            SELECT 
                *
            FROM
                users
            WHERE 
                status = $1
            ORDER BY {} {}
            LIMIT $2 OFFSET $3
            ",
            key, value
        );
    }

    let result: Vec<GetUserDTO> = sqlx::query_as::<_, User>(&query)
        .bind(UserStatus::ACTIVE)
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

pub async fn delete_user_with_status(pool: &PgPool, id: Uuid) -> Result<GetUserDTO, AppError> {
    let result: GetUserDTO = sqlx::query_as::<_, User>(
        r#"--sql
        UPDATE
            users
        SET 
            status = $1,
            updated_at = $2,
            deleted_at = $3
        WHERE 
            id = $4 AND status != $5
        RETURNING 
           *
        "#,
    )
    .bind(UserStatus::DELETED)
    .bind(Utc::now())
    .bind(Utc::now())
    .bind(id)
    .bind(UserStatus::DELETED)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}

pub async fn update_user_password(
    pool: &PgPool,
    id: Uuid,
    payload: UpdateUserPasswordDto,
) -> Result<GetUserDTO, AppError> {
    let User {
        password,
        updated_at,
        ..
    } = payload.into();

    let result: GetUserDTO = sqlx::query_as::<_, User>(
        r#"--sql
        UPDATE
            users
        SET 
            password = $1 
            updated_at = $2
        WHERE 
            id = $3
        RETURNING 
           *
        "#,
    )
    .bind(password)
    .bind(updated_at)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound(format!("User with ID {} not found", id)))?
    .into();

    Ok(result)
}
