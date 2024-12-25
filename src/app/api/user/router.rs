use std::sync::Arc;

use crate::app::{
    api::{extractors::validation_extractor::ValidationExtractor, response::ApiResponse},
    domain::user::usecase::{
        requests::SignupRequest as UserUsecaseSignupRequest, usecase::UserUseCase,
    },
    infrastructure::{di::DiContainer, user::usecase::UserUseCaseImpl},
};
use axum::{routing::post, Extension, Json, Router};

use super::{requests::SignupRequest, responses::SignupResponse};

pub fn user_router(di: DiContainer) -> Router {
    Router::new()
        .route("/users", post(signup))
        .layer(Extension(di.user_usecase))
        .layer(Extension(di.jwt_auth_token))
}

pub async fn signup(
    ValidationExtractor(request): ValidationExtractor<SignupRequest>,
    Extension(user_usecase): Extension<Arc<UserUseCaseImpl>>,
) -> ApiResponse<Json<SignupResponse>> {
    let request = UserUsecaseSignupRequest::from(request);
    let user = user_usecase.signup(request).await?;
    Ok(Json(SignupResponse::from(user)))
}
