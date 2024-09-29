use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    Executor, PgPool, Pool, Postgres,
};

use super::settings::DatabaseSettings;

pub type DbPool = Pool<Postgres>;

pub struct PostgreSQL {
    pool: DbPool,
}

impl PostgreSQL {
    fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    pub async fn configure_database(config: DatabaseSettings) -> Self {
        let ssl_mode = if config.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        let connection = PgConnectOptions::new()
            .host(&config.host)
            .username(&config.username)
            .password(&config.password)
            .port(config.port)
            .database(&config.database_name)
            .ssl_mode(ssl_mode);
        let pool = PgPool::connect_with(connection)
            .await
            .expect("Failed to connect to Postgres");
        PostgreSQL::new(pool)
    }

    pub async fn migrate(&self) {
        let database_name = self
            .pool
            .connect_options()
            .get_database()
            .expect("Failed to get database name")
            .to_owned();

        self.pool
            .execute(format!(r#"CREATE DATABASE "{}";"#, database_name).as_str())
            .await
            .expect("PostgreSQL::migration || error: failed to create database.");

        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .expect("Failed to migrate the database");
    }

    pub fn pool(&self) -> DbPool {
        self.pool.clone()
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}
