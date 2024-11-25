use crate::utils::errors::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params,
};
use tokio::task;

pub async fn hash_password(password: String) -> Result<String, AppError> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);

        let params = Params::new(1024, 1, 4, None)
            .map_err(|e| AppError::PasswordHashingError(e.to_string()))?;
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::PasswordHashingError(e.to_string()))?;

        Ok(password_hash.to_string())
    })
    .await
    .map_err(|e| AppError::PasswordHashingError(e.to_string()))?
}
