use crate::users::entity::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for GetUserDTO {
    fn from(value: User) -> Self {
        GetUserDTO {
            id: value.id.unwrap(),
            name: value.name.unwrap(),
            email: value.email.unwrap(),
            created_at: value.created_at.unwrap(),
            updated_at: value.updated_at.unwrap(),
        }
    }
}
