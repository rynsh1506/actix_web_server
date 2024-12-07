use crate::{auth::dto::Claims, server::AppState};
use actix_web::{web, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use uuid::Uuid;

use super::errors::AppError;

pub fn verify_jwt(req: &HttpRequest, state: &web::Data<AppState>) -> Result<Claims, String> {
    let auth_header = req.headers().get("Authorization").cloned();

    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ").to_string();
                let secret_key = state.secret_key.clone();
                let decoding_key = DecodingKey::from_secret(secret_key.as_ref().as_bytes());
                let validation = Validation::new(Algorithm::HS256);

                match decode::<Claims>(&token, &decoding_key, &validation) {
                    Ok(decoded_token) => Ok(decoded_token.claims),
                    Err(e) => Err(format!("Invalid token: {}", e)),
                }
            } else {
                Err("Authorization header is malformed".into())
            }
        } else {
            Err("Invalid authorization header".into())
        }
    } else {
        Err("Missing Authorization header".into())
    }
}

pub fn verify_refresh_jwt(
    refresh_token: String,
    state: &web::Data<AppState>,
) -> Result<Uuid, AppError> {
    let decoding_key = DecodingKey::from_secret(state.refresh_key.as_ref().as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(&refresh_token, &decoding_key, &validation) {
        Ok(decoded_token) => Ok(decoded_token.claims.sub),
        Err(e) => Err(AppError::Unauthorized(e.to_string())),
    }
}
