use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

fn default_updated_at() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 3, max = 255))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 8))]
    pub password: Option<String>,

    #[serde(default = "default_updated_at")]
    pub updated_at: DateTime<Utc>,
}
