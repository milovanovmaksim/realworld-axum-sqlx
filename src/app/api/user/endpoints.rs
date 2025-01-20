use std::sync::Arc;

use axum::{Extension, Json};

use crate::app::{
    api::{extractors::validation_extractor::ValidationExtractor, response::ApiResponse},
    domain::{
        error::AppError,
        user::usecase::{requests::SignupUserUsecaseRequest, usecase::UserUseCase},
    },
    infrastructure::user::usecase::UserUseCaseImpl,
};

use super::{requests::{SigninUserRequest, SignupUserRequest}, responses::{SignupUserResponse, SigninUserResponse}};

#[utoipa::path(post,
    path = "/api/v1/users",
    tag = "User and Authentication",
    request_body(content = SignupUserRequest, content_type = "application/json"),
    description = "Register a new user",
    responses(
        (status = StatusCode::OK, description = "New user has been created", body = SignupUserResponse, content_type = "application/json"),
        (status = StatusCode::UNPROCESSABLE_ENTITY, description = "Unprocessable entity", body = HashMap<String, HashMap<String, Vec<String>>>,
            content_type = "application/json",
            example = json!({
                "errors": {
                    "email": ["email is invalid"],
                    "password": ["password must be more than 8 characters"]
                }
            })),
        (status = StatusCode::CONFLICT, description = "Conflict", body = HashMap<String, String>,
            content_type = "application/json",
            example=json!({"error": "duplicate key value violates unique constraint \"users_email_key\""})),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    )
)]
pub async fn signup(
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>,
    ValidationExtractor(request): ValidationExtractor<SignupUserRequest>,
) -> ApiResponse<Json<SignupUserResponse>> {
    let request = SignupUserUsecaseRequest::from(request);
    let user = user_usecase.signup(request).await?;
    Ok(Json(SignupUserResponse::from(user)))
}


#[utoipa::path(post,
    path = "/api/v1/users/login",
    tag = "User and Authentication",
    request_body(content = SigninUserRequest, content_type = "application/json"),
    description = "Login for existing user",
    responses(
        (status = StatusCode::OK, description = "User has been logged", body = SigninUserResponse, content_type = "application/json"),
        (status = StatusCode::UNPROCESSABLE_ENTITY, description = "Unprocessable entity", body = HashMap<String, HashMap<String, Vec<String>>>,
            content_type = "application/json",
            example = json!({
                "errors": {
                    "email": ["email is invalid"],
                    "password": ["password must be more than 8 characters"]
                }
            })),
            example=json!({"error": "duplicate key value violates unique constraint \"users_email_key\""})),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    
)]
pub async fn login(
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>,
    ValidationExtractor(request): ValidationExtractor<SigninUserRequest>,
) -> ApiResponse<Json<SigninUserResponse>> {
    let request = SigninUserUsecaseRequest::from(request);
    let user = user_usecase.signin(request).await?;
    Ok(Json(SigninUserResponse::from(user)))
}
