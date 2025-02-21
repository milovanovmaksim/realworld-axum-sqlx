use std::sync::Arc;

use axum::{Extension, Json};
use tracing::info;

use crate::app::{
    api::{extractors::{required_authentication::RequiredAuthentication, validation_extractor::ValidationExtractor}, response::ApiResponse}, domain::user::{self, usecase::UserUseCase}, error::AppError, infrastructure::user::usecase::UserUseCaseImpl
};

use super::{requests::{SigninUserRequest, SignupUserRequest, UpdateUserRequest}, responses::AuthenticationUserResponse};

///
/// Регистрирует нового пользователя.
/// Возвращает информацию о вновь созданном пользователе.
#[utoipa::path(post,
    path = "/api/v1/users",
    tag = "User and Authentication",
    request_body(content = SignupUserRequest, content_type = "application/json"),
    description = "Register a new user",
    responses(
        (status = StatusCode::OK, description = "New user has been created", body = AuthenticationUserResponse,
            content_type = "application/json"),
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
    info!("Recieved request to create new user {:?}/{:?}", request.user.email, request.user.username);

    let user = user_usecase.signup(user::usecase::requests::SignupUserRequest::from(request)).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}



///
/// Авторизация пользователя.
/// Возвращает информайию о пользователе, прошедшего авторизацию.
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
        (status = StausCode::NOT_FOUND, description = "Current user not found", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": "User with email 'example@gmail.com' not found."})),
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
    info!("Recieved request to login a user {:?}", request.user.email);

    let user = user_usecase.login(user::usecase::requests::SigninUserRequest::from(request)).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}


///
/// Возвращает текущего пользователя, прошедшего аутентификацию.
#[utoipa::path(get,
    path = "/api/v1/user",
    tag = "User and Authentication",
    description = "Get current user",
    responses(
        (status = StatusCode::OK, description = "Current user", body = AuthenticationUserResponse, content_type = "application/json"),
        (status = StatusCode::BAD_REQUEST, description = "Bad request", body = HashMap<String, String>,
            content_type = "application/json",
            example=json!({"error": "Token is expired"})),
        (status = StausCode::NOT_FOUND, description = "Current user not found", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": "User with email 'example@gmail.com' not found."})),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ApiResponse<Json<AuthenticationUserResponse>> {
    info!("Recieved request to retrieve current user");

    let user = user_usecase.get_current_user(user_id).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}


///
/// Обновляет информацию о текущем пользователе.
#[utoipa::path(put,
    path = "/api/v1/user",
    tag = "User and Authentication",
    request_body(content = UpdateUserRequest, content_type = "application/json"),
    description = "Update current user",
    responses(
        (status = StatusCode::OK, description = "Current user", body = AuthenticationUserResponse, content_type = "application/json"),
        (status = StatusCode::BAD_REQUEST, description = "Bad request", body = HashMap<String, String>,
            content_type = "application/json",
            example=json!({"error": "Token is expired"})),
        (status = StausCode::NOT_FOUND, description = "Current user not found", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": "User with email 'example@gmail.com' not found."})),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_user(
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>,
    RequiredAuthentication(user_id): RequiredAuthentication,
    ValidationExtractor(request): ValidationExtractor<UpdateUserRequest>,
) -> ApiResponse<Json<AuthenticationUserResponse>> {
    info!("Recieved request to update current user");

    let user = user_usecase.update_user((user_id, user::usecase::requests::UpdateUserRequest::from(request))).await?;
    Ok(Json(AuthenticationUserResponse::from(user)))
}
