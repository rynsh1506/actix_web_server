use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct JwtDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}
