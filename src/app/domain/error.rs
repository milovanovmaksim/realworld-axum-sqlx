use bcrypt::BcryptError;
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde_json::{json, Value as JsonValue};
use sqlx::error::ErrorKind;
use sqlx::Error as DbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // 401
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {0}")]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

impl From<sqlx::Error> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Database(db_err) => match db_err.kind() {
                ErrorKind::UniqueViolation => {
                    let message = db_err.message();
                    AppError::UnprocessableEntity(json!({"error": message}))
                }
                _ => AppError::InternalServerError,
            },
            DbError::RowNotFound => {
                AppError::NotFound(json!({"error": "requested record was not found"}))
            }
            _ => AppError::InternalServerError,
        }
    }
}

impl From<BcryptError> for AppError {
    fn from(_err: BcryptError) -> Self {
        AppError::InternalServerError
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            JwtErrorKind::InvalidToken => AppError::Unauthorized(json!({
                "error": "Token is invalid"
            })),
            JwtErrorKind::InvalidIssuer => AppError::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => AppError::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}
