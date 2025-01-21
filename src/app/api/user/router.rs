use crate::app::infrastructure::di::DiContainer;
use axum::{
    routing::{get, post},
    Extension, Router,
};

use super::endpoints::{get_current_user, login, signup};

pub fn user_router(di: DiContainer) -> Router {
    Router::new()
        .route("/users", post(signup))
        .route("/users/login", post(login))
        .route("/user", get(get_current_user))
        .layer(Extension(di.user_usecase))
        .layer(Extension(di.jwt_auth_token))
}
