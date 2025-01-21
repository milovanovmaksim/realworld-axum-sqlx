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
    // 400
    #[error("{0}")]
    BadRequest(String),

    // 401
    #[error("{0}")]
    Unauthorized(String),

    // 403
    #[error("User does not have privilege to access this resource")]
    Forbidden,

    // 404
    #[error("Requested record was not found")]
    NotFound,

    // 409
    #[error("{0}")]
    Conflict(String),

    // 422
    #[error("{0}")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal server error")]
    InternalServerError,

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
}

impl From<sqlx::Error> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Database(db_err) => match db_err.kind() {
                ErrorKind::UniqueViolation => AppError::Conflict(db_err.message().to_string()),
                _ => AppError::InternalServerError,
            },
            DbError::RowNotFound => AppError::NotFound,
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
            JwtErrorKind::InvalidToken => AppError::Unauthorized(String::from("Token is invalid")),
            JwtErrorKind::InvalidIssuer => {
                AppError::Unauthorized(String::from("Issuer is invalid"))
            }
            JwtErrorKind::ExpiredSignature => {
                AppError::BadRequest(String::from("Token is expired"))
            }
            _ => AppError::Unauthorized(String::from(
                "Authentication is required to access this resource",
            )),
        }
    }
}

impl AppError {
    fn unprocessable_entity(errors: ValidationErrors) -> (StatusCode, Json<JsonValue>) {
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
        let body = Json(json!({"errors": validation_errors}));
        (StatusCode::UNPROCESSABLE_ENTITY, body)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Unauthorized(v) => (StatusCode::UNAUTHORIZED, Json(json!({"error": v}))),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(json!({"error": AppError::Forbidden.to_string()})),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": AppError::NotFound.to_string()})),
            ),
            AppError::BadRequest(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e}))),
            AppError::UnprocessableEntity(v) => (StatusCode::UNPROCESSABLE_ENTITY, Json(v)),
            AppError::ValidationError(e) => Self::unprocessable_entity(e),
            AppError::Conflict(e) => (StatusCode::CONFLICT, Json(json!({"error": e}))),
            AppError::AxumJsonRejection(e) => (e.status(), Json(json!({"error": e.body_text()}))),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": AppError::InternalServerError.to_string() })),
            ),
        };

        (status, error_message).into_response()
    }
}
