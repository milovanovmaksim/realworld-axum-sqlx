use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};

use super::endpoints::{follow_user, get_profile, unfollow_user};
use crate::app::infrastructure::di::DiContainer;

pub fn profile_router(di: Arc<DiContainer>) -> Router {
    Router::new()
        .route("/profiles/{username}", get(get_profile))
        .route("/profiles/{username}/follow", post(follow_user))
        .route("/profiles/{username}/follow", delete(unfollow_user))
        .layer(Extension(di.user_usecase.clone()))
        .layer(Extension(di.profile_usecase.clone()))
        .layer(Extension(di.jwt_auth_token.clone()))
}
