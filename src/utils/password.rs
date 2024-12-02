use crate::utils::errors::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params, PasswordHash, PasswordVerifier,
};

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
        .map_err(|_| AppError::InvalidCredentials("Invalid credentials".to_string()))
}
