use crate::{
    users::entity::{users_model::UserStatus, User},
    utils::password::validate_password,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 3, max = 255))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

impl From<CreateUserDTO> for User {
    fn from(value: CreateUserDTO) -> Self {
        User {
            id: Some(Uuid::new_v4()),
            name: Some(value.name),
            password: Some(value.password),
            email: Some(value.email),
            status: Some(UserStatus::ACTIVE),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }
}
