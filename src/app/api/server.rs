use super::{config::ApiConfig, user::router::user_router};
use crate::app::infrastructure::di::DiContainer;
use axum::{http::HeaderValue, Router};
use tower::ServiceBuilder;
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};

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
        let router = Router::new()
            .nest("/api/v1", user_router(self.di_container.clone()))
            .layer(
                ServiceBuilder::new()
                    // High level logging of requests and responses
                    .layer(TraceLayer::new_for_http()),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(
                        self.api_config
                            .frontend_origin
                            .parse::<HeaderValue>()
                            .unwrap(),
                    )
                    .allow_methods(Any),
            );

        let tcp_listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            self.api_config.host, self.api_config.port
        ))
        .await
        .map_err(|e| format!("Server::start || error: failed to parse soket addres {e}"))?;

        axum::serve(tcp_listener, router)
            .await
            .map_err(|e| format!("Server::start || error: failed to start server {e}"))?;

        Ok(())
    }
}
