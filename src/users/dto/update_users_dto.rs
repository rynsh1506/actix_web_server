use crate::{
    users::entity::{User, UserStatus},
    utils::password::validate_password,
};
use chrono::Utc;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 3, max = 255))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    pub status: Option<UserStatus>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserPasswordDto {
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

impl From<UpdateUserDTO> for User {
    fn from(value: UpdateUserDTO) -> Self {
        User {
            id: None,
            name: value.name,
            email: value.email,
            password: None,
            status: value.status,
            created_at: None,
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }
}

impl From<UpdateUserPasswordDto> for User {
    fn from(value: UpdateUserPasswordDto) -> Self {
        User {
            id: None,
            name: None,
            email: None,
            password: Some(value.password),
            status: None,
            created_at: None,
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }
}
