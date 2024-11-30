use super::time::custom_timezone_with_fromat;
use actix_web::{
    error::{JsonPayloadError, QueryPayloadError},
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, ResponseError,
};
use serde::Serialize;
use sqlx::Error as SqlxError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound(String),

    #[error("Unauthorized access")]
    Unauthorized(String),

    #[error("Bad request")]
    BadRequest(String),

    #[error("Duplicate entry")]
    Conflict(String),

    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),

    #[error("Internal server error")]
    InternalServerError(String),

    #[error("Database error")]
    DatabaseError(#[from] SqlxError),

    #[error("Timeout error")]
    TimeoutError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded(String),

    #[error("Password hashing error")]
    PasswordHashingError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.to_string(),
            message: match self {
                AppError::NotFound(err) => err.to_string(),
                AppError::Unauthorized(err) => err.to_string(),
                AppError::BadRequest(err) => err.to_string(),
                AppError::InternalServerError(err) => err.to_string(),
                AppError::ValidationError(errors) => format_validation_errors(errors),
                AppError::DatabaseError(err) => err.to_string(),
                AppError::TimeoutError(err) => err.to_string(),
                AppError::RateLimitExceeded(err) => err.to_string(),
                AppError::PasswordHashingError(err) => err.to_string(),
                AppError::Conflict(err) => err.to_string(),
            },
            code: self.status_code().as_u16(),
            timestamp: custom_timezone_with_fromat(),
        };

        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TimeoutError(_) => StatusCode::REQUEST_TIMEOUT,
            AppError::RateLimitExceeded(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    code: u16,
    timestamp: String,
}

fn format_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .flat_map(|(field, errs)| {
            errs.iter().map(move |err| {
                let message = err
                    .message
                    .clone()
                    .unwrap_or_else(|| "Invalid value".into());
                format!("{}: {}", field, message)
            })
        })
        .collect::<Vec<String>>()
        .join("; ")
}

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    AppError::BadRequest(err.to_string()).into()
}

pub fn query_error_handler(err: QueryPayloadError, _req: &HttpRequest) -> actix_web::Error {
    AppError::BadRequest(err.to_string()).into()
}
