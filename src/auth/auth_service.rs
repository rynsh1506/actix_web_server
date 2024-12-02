use actix_web::web;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::dto::{jwt_dto::JwtDto, Claims, LoginDto},
    server::AppState,
    users::{dto::CreateUserDTO, users_query},
    utils::{
        errors::AppError,
        password::{hash_password, verify_password},
        response_data::ResponseData,
    },
};

pub async fn register(
    pool: &PgPool,
    app_state: &web::Data<AppState>,
    mut payload: CreateUserDTO,
) -> Result<ResponseData<JwtDto>, AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    payload.password = hash_password(&payload.password)?;

    let user_id = users_query::create_user_query(pool, payload).await?;

    let access_token = generate_token(user_id, app_state)?;
    let refresh_token = generate_refresh_token(user_id, app_state)?;

    Ok(ResponseData::new(JwtDto {
        access_token,
        refresh_token,
    }))
}

pub async fn login(
    pool: &PgPool,
    app_state: &web::Data<AppState>,
    payload: LoginDto,
) -> Result<ResponseData<JwtDto>, AppError> {
    let LoginDto { email, password } = payload;

    let result = users_query::login_users_query(pool, &email).await?;

    verify_password(&password, &result.password)?;

    let access_token = generate_token(result.id, app_state)?;
    let refresh_token = generate_refresh_token(result.id, app_state)?;

    Ok(ResponseData::new(JwtDto {
        access_token,
        refresh_token,
    }))
}

pub async fn refresh() -> Result<ResponseData<JwtDto>, AppError> {
    todo!()
}

fn generate_token(user_id: Uuid, app_state: &web::Data<AppState>) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(*app_state.jwt_expiration_time)
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.secret_key.as_bytes()),
    )
    .map_err(|err| AppError::InternalServerError(err.to_string()))
}

fn generate_refresh_token(
    user_id: Uuid,
    app_state: &web::Data<AppState>,
) -> Result<String, AppError> {
    let refresh_expiration = chrono::Utc::now()
        .checked_add_signed(*app_state.jwt_refresh_expiration_time)
        .expect("Valid timestamp")
        .timestamp() as usize;

    let refresh_claims = Claims {
        sub: user_id,
        exp: refresh_expiration,
    };

    encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(app_state.secret_key.as_bytes()),
    )
    .map_err(|err| AppError::InternalServerError(err.to_string()))
}
