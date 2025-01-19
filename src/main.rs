use app::{
    api::{config::ApiConfig, server::Server},
    infrastructure::di::DiContainer,
};

mod app;
mod tests;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let di_container = DiContainer::new("./config/config.yaml")
        .await
        .expect("Failed to create di container");
    let api_config =
        ApiConfig::from_yaml("./configuration/config.yaml").expect("Failed to create api config");
    let server = Server::new(di_container, api_config);
    server.start().await;
}
