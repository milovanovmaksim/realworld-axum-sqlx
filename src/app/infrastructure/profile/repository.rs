use std::sync::Arc;

use async_trait::async_trait;
use sqlx::query;
use tracing::info;

use crate::app::{
    domain::profile::repository::ProfileRepository, error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};

pub struct ProfileRepositoryImpl {
    pg_sql: Arc<PostgreSQL>,
}

impl ProfileRepositoryImpl {
    pub fn new(pg_sql: Arc<PostgreSQL>) -> Self {
        Self { pg_sql }
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositoryImpl {
    async fn is_follower(
        &self,
        current_user_id: uuid::Uuid,
        followee_user_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        info!("Searching following user");
        let record = query!(
            r#"
            select 1 as "id?" from user_follows where follower_id = $1 and followee_id = $2
            "#,
            current_user_id,
            followee_user_id
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(record.is_some())
    }
}
