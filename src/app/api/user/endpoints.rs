use std::sync::Arc;

use axum::{Extension, Json};
use tracing::info;

use crate::app::{
    api::{extractors::{required_authentication::RequiredAuthentication, validation_extractor::ValidationExtractor}, response::ApiResponse},
    domain::{
        error::AppError,
        user::usecase::{requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest}, usecase::UserUseCase},
    },
    infrastructure::user::usecase::UserUseCaseImpl,
};

use super::{requests::{SigninUserRequest, SignupUserRequest}, responses::AuthenticationUserResponse};

#[utoipa::path(post,
    path = "/api/v1/users",
    tag = "User and Authentication",
    request_body(content = SignupUserRequest, content_type = "application/json"),
    description = "Register a new user",
    responses(
        (status = StatusCode::OK, description = "New user has been created", body = AuthenticationUserResponse, content_type = "application/json"),
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
) -> ApiResponse<Json<AuthenticationUserResponse>> {
    info!("recieved request to create new user {:?}/{:?}", request.user.email, request.user.username);

    let request = SignupUserUsecaseRequest::from(request);
    let user = user_usecase.signup(request).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}


#[utoipa::path(post,
    path = "/api/v1/users/login",
    tag = "User and Authentication",
    request_body(content = SigninUserRequest, content_type = "application/json"),
    description = "Login for existing user",
    responses(
        (status = StatusCode::OK, description = "User has been logged", body = AuthenticationUserResponse, content_type = "application/json"),
        (status = StatusCode::BAD_REQUEST, description = "Bad request", body = HashMap<String, String>,
                    content_type = "application/json",
                    example=json!({"error": "password is incorrect"})),
        (status = StatusCode::UNPROCESSABLE_ENTITY, description = "Unprocessable entity", body = HashMap<String, HashMap<String, Vec<String>>>,
            content_type = "application/json",
            example = json!({
                "errors": {
                    "email": ["email is invalid"],
                    "password": ["password must be more than 8 characters"]
                }
            })),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
        )
    
)]
pub async fn login(
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>,
    ValidationExtractor(request): ValidationExtractor<SigninUserRequest>,
) -> ApiResponse<Json<AuthenticationUserResponse>> {
    info!("recieved request to login a user {:?}", request.user.email);

    let request = SigninUserUsecaseRequest::from(request);
    let user = user_usecase.login(request).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}



pub async fn get_current_user(
    RequiredAuthentication(user_id): RequiredAuthentication,
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>
) -> ApiResponse<Json<AuthenticationUserResponse>> {
    info!("recieved request to retrieve current user");

    let user = user_usecase.get_current_user(user_id).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}
