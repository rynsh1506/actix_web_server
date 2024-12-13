use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct GetLoginDto {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}
