use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use crate::app::infrastructure::di::DiContainer;

use super::endpoints::get_tags;

///
/// Возвращает маршрутизатор для тэгов.
pub fn tags_router(di: Arc<DiContainer>) -> Router {
    Router::new()
        .route("/tags", get(get_tags))
        .layer(Extension(di.tags_usecase.clone()))
}
