use std::borrow::Cow;
use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::BcryptError;
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde_json::{json, Value as JsonValue};
use sqlx::error::ErrorKind;
use sqlx::Error as DbError;
use thiserror::Error;
use validator::{ValidationErrors, ValidationErrorsKind};

pub type AppErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;

#[derive(Error, Debug)]
pub enum AppError {
    // 401
    #[error("{0}")]
    Unauthorized(JsonValue),

    // 403
    #[error("User does not have privilege to access this resource")]
    Forbidden,

    // 404
    #[error("{0}")]
    NotFound(JsonValue),

    // 422
    #[error("{0}")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
}

impl From<sqlx::Error> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Database(db_err) => match db_err.kind() {
                ErrorKind::UniqueViolation => {
                    let message = db_err.message();
                    AppError::UnprocessableEntity(json!({"message": message}))
                }
                _ => AppError::InternalServerError,
            },
            DbError::RowNotFound => {
                AppError::NotFound(json!({"message": "Requested record was not found"}))
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
                "message": "Token is invalid"
            })),
            JwtErrorKind::InvalidIssuer => AppError::Unauthorized(json!({
                "message": "Issuer is invalid",
            })),
            _ => AppError::Unauthorized(json!({
                "message": "Authentication is required to access this resource",
            })),
        }
    }
}

impl AppError {
    fn unprocessable_entity(errors: ValidationErrors) -> Response {
        let mut validation_errors = AppErrorMap::new();

        for (_, error_kind) in errors.into_errors() {
            if let ValidationErrorsKind::Struct(meta) = error_kind {
                for (struct_property, struct_error_kind) in meta.into_errors() {
                    if let ValidationErrorsKind::Field(field_meta) = struct_error_kind {
                        for error in field_meta.into_iter() {
                            validation_errors
                                .entry(Cow::from(struct_property))
                                .or_insert_with(Vec::new)
                                .push(error.message.unwrap_or_else(|| {
                                    Cow::from(format!("{} is required", struct_property))
                                }));
                        }
                    }
                }
            }
        }
        let body = Json(json!({"error": validation_errors}));
        (StatusCode::UNPROCESSABLE_ENTITY, body).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let Self::ValidationError(e) = self {
            return Self::unprocessable_entity(e);
        }

        let (status, error_message) = match self {
            AppError::Unauthorized(v) => (StatusCode::UNAUTHORIZED, Json(v)),
            AppError::Forbidden => (StatusCode::FORBIDDEN, Json(json!({"message": AppError::Forbidden.to_string()}))),
            AppError::NotFound(v) => (StatusCode::NOT_FOUND, Json(v)),
            AppError::UnprocessableEntity(v) =>(StatusCode::UNPROCESSABLE_ENTITY, Json(v)),
            _ => todo!(),
        };
        todo!()
    }
}
