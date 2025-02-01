use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{query, query_as};
use tracing::info;
use uuid::Uuid;

use crate::app::{
    domain::profile::repository::{entities::UserFollow, ProfileRepository},
    error::AppError,
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
        followee_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        info!("Searching following user");
        let record = query!(
            r#"
            select 1 as "id?" from user_follows where follower_id = $1 and followee_id = $2
            "#,
            current_user_id,
            followee_id
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(record.is_some())
    }

    async fn add_user_follow(
        &self,
        current_user_id: Uuid,
        followee_id: Uuid,
    ) -> Result<UserFollow, AppError> {
        info!("Query of creating user follow.");
        let user_follow = query_as!(
            UserFollow,
            r#"
            insert into user_follows (follower_id, followee_id)
            values ($1, $2) returning *;
            "#,
            current_user_id,
            followee_id
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user_follow)
    }
}
