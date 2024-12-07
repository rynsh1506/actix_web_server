use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

pub struct GetLoginDto {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}
