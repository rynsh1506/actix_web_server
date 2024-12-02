use crate::{auth::dto::Claims, utils::errors::AppError};
use actix_web::{HttpMessage, HttpRequest};
use std::sync::Arc;
use uuid::Uuid;

pub fn validate_user_id_in_token(req: &HttpRequest, user_id: &Uuid) -> Result<(), AppError> {
    let extensions = req.extensions();

    let claims = extensions
        .get::<Arc<Claims>>()
        .ok_or(AppError::Unauthorized("Invalid JWT claims".to_string()))?;

    log::info!("{} {}", claims.sub, user_id);

    if claims.sub != *user_id {
        return Err(AppError::Unauthorized(
            "Not authorized to access this resource".to_string(),
        ));
    }

    Ok(())
}
