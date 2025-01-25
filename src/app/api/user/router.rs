use crate::app::infrastructure::di::DiContainer;
use axum::{
    routing::{get, post, put},
    Extension, Router,
};

use super::endpoints::{get_current_user, login, signup, update_user};

pub fn user_router(di: DiContainer) -> Router {
    Router::new()
        .route("/users", post(signup))
        .route("/users/login", post(login))
        .route("/user", get(get_current_user))
        .route("/user", put(update_user))
        .layer(Extension(di.user_usecase))
        .layer(Extension(di.jwt_auth_token))
}
