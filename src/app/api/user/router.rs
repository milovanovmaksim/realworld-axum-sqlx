use std::sync::Arc;

use crate::app::{
    api::{extractors::validation_extractor::ValidationExtractor, response::ApiResponse},
    domain::{
        error::AppError,
        user::usecase::{requests::SignupUserUsecaseRequest, usecase::UserUseCase},
    },
    infrastructure::{di::DiContainer, user::usecase::UserUseCaseImpl},
};
use axum::{routing::post, Extension, Json, Router};

use super::{requests::SignupUserRequest, responses::SignupUserResponse};

pub fn user_router(di: DiContainer) -> Router {
    Router::new()
        .route("/users", post(signup))
        .layer(Extension(di.user_usecase))
        .layer(Extension(di.jwt_auth_token))
}

#[utoipa::path(post,
    path = "/api/v1/users",
    tag = "User and  Authentication",
    request_body(content = SignupUserRequest, content_type = "applicationjson"),
    description = "Register a new user",
    responses(
        (status = StatusCode::OK, description = "New user has been created", body = SignupUserResponse, content_type = "application/json"),
        (status = StatusCode::UNPROCESSABLE_ENTITY, description = "Unprocessable entity", body = HashMap<String, HashMap<String, Vec<String>>>,
            content_type = "application/json",
            example = json!({
                "errors": {
                    "body": ["body is required"]
                }
            })),
        (status = StatusCode::CONFLICT, description = "Conflict", body = HashMap<String, String>,
            content_type = "application/json",
            example=json!({"error": "User with email 'example@gmail.com' allredy exists"})),
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
