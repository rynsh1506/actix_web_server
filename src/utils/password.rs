use crate::utils::errors::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params, PasswordHash, PasswordVerifier,
};
use regex::Regex;
use validator::ValidationError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let params =
        Params::new(1024, 1, 4, None).map_err(|e| AppError::PasswordHashingError(e.to_string()))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::PasswordHashingError(e.to_string()))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|e| AppError::PasswordHashingError(e.to_string()))?;

    let argon2 = Argon2::default();

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| AppError::InvalidCredentials(e.to_string()))
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let lowercase = Regex::new(r"[a-z]").unwrap();
    let uppercase = Regex::new(r"[A-Z]").unwrap();
    let digit = Regex::new(r"\d").unwrap();
    let special_char = Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':\"\\|,.<>\/?]"#).unwrap();

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

    if !special_char.is_match(password) {
        let mut error = ValidationError::new("password_special_char");
        error.message =
            Some("Password must contain at least one special character (e.g., !@#$%^&*).".into());
        return Err(error);
    }

    Ok(())
}
