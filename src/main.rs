use std::sync::Arc;

use app::{
    api::{config::ApiConfig, server::Server},
    infrastructure::di::DiContainer,
};

mod app;

#[tokio::main]
async fn main() {
    let di_container = DiContainer::new("./config/config.yaml")
        .await
        .expect("Failed to create di container");
    let api_config =
        ApiConfig::from_yaml("./config/config.yaml").expect("Failed to create api config");
    let server = Server::new(Arc::new(di_container), api_config);
    server.start().await.expect("Failed to run server");
}
