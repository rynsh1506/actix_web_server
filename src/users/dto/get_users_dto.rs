use crate::users::entity::{users_model::UserStatus, User};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct GetUserDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<User> for GetUserDTO {
    fn from(value: User) -> Self {
        GetUserDTO {
            id: value.id.unwrap(),
            name: value.name.unwrap(),
            email: value.email.unwrap(),
            status: value.status.unwrap(),
            created_at: value.created_at.unwrap(),
            updated_at: value.updated_at.unwrap(),
            deleted_at: value.deleted_at,
        }
    }
}
