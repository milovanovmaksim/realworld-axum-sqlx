use crate::app::infrastructure::di::DiContainer;
use axum::{routing::post, Extension, Router};

use super::endpoints::signup;

pub fn user_router(di: DiContainer) -> Router {
    Router::new()
        .route("/users", post(signup))
        .layer(Extension(di.user_usecase))
        .layer(Extension(di.jwt_auth_token))
}
