use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 3, max = 255))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[serde(default = "default_timestamp")]
    pub created_at: DateTime<Utc>,

    #[serde(default = "default_timestamp")]
    pub updated_at: DateTime<Utc>,
}
