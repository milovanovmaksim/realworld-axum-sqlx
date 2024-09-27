use std::env;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

use super::constants::env_key;

pub type DbPool = Pool<Postgres>;

async fn init_pool(database_url: &str) -> Result<DbPool, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE url must be set");
    init_pool(&database_url)
        .await
        .expect("Failed to create pool")
}
