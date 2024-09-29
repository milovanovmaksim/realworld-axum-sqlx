use dotenv::dotenv;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::env;

use crate::app::constants::env_key;

pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn new(
        username: String,
        password: String,
        port: u16,
        host: String,
        database_name: String,
        require_ssl: bool,
    ) -> Self {
        Self {
            username,
            password,
            port,
            host,
            database_name,
            require_ssl,
        }
    }

    pub fn from_env(require_ssl: bool) -> Self {
        dotenv().ok();
        let password = env::var(env_key::DATABASE_PASSWORD).expect("DATABASE_PASSWORD must be set");
        let username = env::var(env_key::DATABASE_USERNAME).expect("DATABASE_USERNAME must be set");
        let port = env::var(env_key::DATABASE_PORT)
            .expect("DATABASE_PORT must be set")
            .parse::<u16>()
            .expect("failed to parse DATABASE_PORT");
        let host = env::var(env_key::DATABASE_HOST).expect("DATABASE_HOST must be set");
        let database_name = env::var(env_key::DATABASE_NAME).expect("DATABASE_NAME must be set");
        Self {
            username,
            password,
            port,
            host,
            database_name,
            require_ssl,
        }
    }
}
