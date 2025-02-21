use std::ops::Deref;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Executor, Pool, Postgres,
};

use super::settings::DatabaseSettings;

pub type DbPool = Pool<Postgres>;

///
/// Клиент для работы с PostgreSQL.
pub struct PostgreSQL {
    pool: DbPool,
}

impl PostgreSQL {
    ///
    /// Основной конструктор.
    fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    ///
    /// Создает объект [PostgreSQL] из DatabaseSettings.
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
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(connection)
            .await
            .expect("PostgreSQL::configure_database || error: failed to connect to Postgres");

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

    ///
    /// Возвращает пул соединений.
    pub fn pool(&self) -> DbPool {
        self.pool.clone()
    }
}

impl Deref for PostgreSQL {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl Clone for PostgreSQL {
    /// Returns a new [PostgreSQL] tied to the same shared connection pool.
    fn clone(&self) -> Self {
        PostgreSQL::new(self.pool.clone())
    }
}
