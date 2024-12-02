use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

pub struct GetLoginDto {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}
