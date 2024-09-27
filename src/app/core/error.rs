use bcrypt::BcryptError;
use serde_json::{json, Value as JsonValue};
use sqlx::error::ErrorKind;
use sqlx::Error as DbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // 401
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

impl From<DbError> for AppError {
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
