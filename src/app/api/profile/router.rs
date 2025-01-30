use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use crate::app::infrastructure::di::DiContainer;
use super::endpoints::get_profile

pub fn profile_router(di: Arc<DiContainer>) -> Router {
    Router::new()
        .route("/profiles/:username", get(get_profile))
        .layer(Extension(di.user_usecase.clone()))
        .layer(Extension(di.jwt_auth_token.clone()))
}
