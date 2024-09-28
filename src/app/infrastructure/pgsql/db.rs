use sqlx::{PgPool, Pool, Postgres};

use super::settings::DatabaseSettings;

pub type DbPool = Pool<Postgres>;

pub async fn establish_connection(config: DatabaseSettings) -> DbPool {
    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    pool
}
