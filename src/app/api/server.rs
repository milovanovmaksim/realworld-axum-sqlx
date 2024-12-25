use axum::Router;

use crate::app::infrastructure::di::DiContainer;

use super::{config::ApiConfig, user::router::user_router};

pub struct Server {
    api_config: ApiConfig,
    di_container: DiContainer,
}

impl Server {
    pub fn new(di_container: DiContainer, api_config: ApiConfig) -> Self {
        Self {
            di_container,
            api_config,
        }
    }
    pub fn start(&self) {
        let router = Router::new().nest("/api", user_router(self.di_container.clone()));
    }
}
