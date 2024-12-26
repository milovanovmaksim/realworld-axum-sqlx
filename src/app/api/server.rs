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
    pub async fn start(&self) -> Result<(), String> {
        let router = Router::new().nest("/api", user_router(self.di_container.clone()));

        let soket_addr = &format!("{}:{}", self.api_config.host, self.api_config.port)
            .parse()
            .map_err(|e| format!("Server::start || error: failed to parse soket addres {e}"))?;

        axum::Server::bind(&soket_addr)
            .serve(router.into_make_service())
            .await
            .map_err(|e| format!("Server::start || error: failed to start server {e}"))?;
        Ok(())
    }
}
