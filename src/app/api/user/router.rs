use crate::app::infrastructure::di::DiContainer;
use axum::{routing::post, Extension, Router};

use super::endpoints::{login, signup};

pub fn user_router(di: DiContainer) -> Router {
    Router::new()
        .route("/users", post(signup))
        .route("/users/login", post(login))
        .layer(Extension(di.user_usecase))
        .layer(Extension(di.jwt_auth_token))
}
