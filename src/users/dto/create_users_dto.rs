use crate::users::entity::User;
use chrono::Utc;
use regex::Regex;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let lowercase = Regex::new(r"[a-z]").unwrap();
    let uppercase = Regex::new(r"[A-Z]").unwrap();
    let digit = Regex::new(r"\d").unwrap();

    if password.len() < 8 {
        let mut error = ValidationError::new("password_length");
        error.message = Some("Password must be at least 8 characters long.".into());
        return Err(error);
    }

    if !lowercase.is_match(password) {
        let mut error = ValidationError::new("password_lowercase");
        error.message = Some("Password must contain at least one lowercase letter (a-z).".into());
        return Err(error);
    }

    if !uppercase.is_match(password) {
        let mut error = ValidationError::new("password_uppercase");
        error.message = Some("Password must contain at least one uppercase letter (A-Z).".into());
        return Err(error);
    }

    if !digit.is_match(password) {
        let mut error = ValidationError::new("password_digit");
        error.message = Some("Password must contain at least one digit (0-9).".into());
        return Err(error);
    }

    Ok(())
}

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
            email: Some(value.email),
            password: Some(value.password),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}
