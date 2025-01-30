use std::sync::Arc;

use crate::app::{
    domain::user::{
        self,
        repository::{entities, Email, UserRepository},
    },
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};

use async_trait::async_trait;
use sqlx::{query_as, query_file_as};
use tracing::info;
use uuid::Uuid;

pub struct UsersRepositoryImpl {
    pg_sql: Arc<PostgreSQL>,
}

impl UsersRepositoryImpl {
    pub fn new(pg_sql: Arc<PostgreSQL>) -> UsersRepositoryImpl {
        UsersRepositoryImpl { pg_sql }
    }
}

#[async_trait]
impl UserRepository for UsersRepositoryImpl {
    async fn login(
        &self,
        request: user::repository::requests::SigninUserRequest,
    ) -> Result<Option<entities::User>, AppError> {
        self.get_user_by_email(request.email).await
    }

    async fn signup(
        &self,
        request: user::repository::requests::SignupUserRequest,
    ) -> Result<entities::User, AppError> {
        info!(
            "Creating new user {:?}/{:?}",
            request.email, request.username
        );

        let user = query_file_as!(
            entities::User,
            "./src/app/infrastructure/queries/users/insert.sql",
            request.username,
            request.email,
            request.hashed_password
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: Email) -> Result<Option<entities::User>, AppError> {
        info!("Searching for user by email in db {:?}", email);

        let user = query_as!(
            entities::User,
            r#"select * from users where email = $1"#,
            email,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<entities::User>, AppError> {
        info!("Searching for user by id in db {:?}", user_id);

        let user = query_as!(
            entities::User,
            r#"select * from users where id = $1"#,
            user_id,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
        
        }

    async fn get_user_by_username(&self, username: String) -> Result<Option<entities::User>, AppError> {
        info!("Searching for user by username in db {:?}", username);

        let user = query_as!(
            entities::User,
            r#"select * from users where username = $1"#,
            username,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
        
    }

    async fn update_user(
        &self,
        request: user::repository::requests::UpdateUserRequest,
    ) -> Result<entities::User, AppError> {
        info!("Updating user");

        let user = query_file_as!(
            entities::User,
            "./src/app/infrastructure/queries/users/update.sql",
            request.username,
            request.email,
            request.hashed_password,
            request.bio,
            request.image,
            request.id
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }
}
