use crate::app::{
    api::extractors::validation_extractor::ValidationExtractor,
    infrastructure::{di::DiContainer, user::usecase::UserUseCaseImpl},
};
use axum::{routing::post, Extension, Router};

use super::requests::Signup;

pub fn user_router(di: DiContainer) -> Router {
    Router::new().route("/users", post(signup))
}

pub async fn signup(
    ValidationExtractor(request): ValidationExtractor<Signup>,
    Extension(user_usecase): Extension<UserUseCaseImpl>,
) {
}
