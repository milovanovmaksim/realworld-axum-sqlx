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

///
/// ProfileRepositoryImpl реализует интерфейс ProfileRepository для работы с таблицей 'user_follows' базы данных PostgreSQL.
/// pg_sql - клиент для работы с PostgreSQL.
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
    ///
    /// Создает новую запись в таблицу user_follows.
    /// follower_id - id пользователя;
    /// followee_id - id пользователя на которого подписываем follower_id.
    async fn add_user_follow(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> Result<UserFollow, AppError> {
        info!("Query of creating user follow.");
        let user_follow = query_as!(
            UserFollow,
            r#"
            insert into user_follows (follower_id, followee_id)
            values ($1, $2) returning *;
            "#,
            follower_id,
            followee_id
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user_follow)
    }

    ///
    /// Возвращает true, если пользователь user_id является подписчиком пользователя followee_id.
    /// user_id - id пользователя;
    /// followee_id - id пользователя на которого возможно подписан текущий пользователь.
    async fn is_follower(
        &self,
        user_id: uuid::Uuid,
        followee_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        info!("Query 'is follower?'");
        let record = query!(
            r#"
            select 1 as "id?" from user_follows where follower_id = $1 and followee_id = $2
            "#,
            user_id,
            followee_id
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(record.is_some())
    }

    ///
    /// Удаляет запись из таблицы user_follows, если такая запись существует.
    /// follower_id - id пользователя(подписчика);
    /// followee_id - id пользователя от которого хотим отписаться.
    async fn remove_user_follow(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> Result<(), AppError> {
        info!("Query to remove user follow.");
        query!(
            r#"
            delete from user_follows where (follower_id, followee_id) = ($1, $2);
            "#,
            follower_id,
            followee_id
        )
        .execute(&self.pg_sql.pool())
        .await?;
        Ok(())
    }
}
