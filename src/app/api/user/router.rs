use crate::app::infrastructure::{di::DiContainer, user::usecase::UserUseCaseImpl};
use axum::{routing::post, Extension, Router};

pub fn user_router(di: DiContainer) -> Router {
    Router::new().route("/users", post(signup))
}

pub async fn signup(
    ValidationExtractor(request): ValidationExtractor<Signup>,
    Extension(users_service): Extension<UserUseCaseImpl>,
) {
}
