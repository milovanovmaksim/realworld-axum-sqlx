use crate::app::{
    domain::{
        error::AppError,
        user::repository::{
            entities::User,
            repository::UserRepository,
            requests::{SigninUserRepositoryRequest, SignupUserRepositoryRequest},
        },
    },
    infrastructure::pgsql::db::PostgreSQL,
};

use async_trait::async_trait;
use sqlx::{query_as, query_file_as};

pub struct UsersRepositoryImpl {
    pg_sql: PostgreSQL,
}

impl UsersRepositoryImpl {
    pub fn new(pg_sql: PostgreSQL) -> UsersRepositoryImpl {
        UsersRepositoryImpl { pg_sql }
    }
}

#[async_trait]
impl UserRepository for UsersRepositoryImpl {
    async fn login(&self, request: SigninUserRepositoryRequest) -> Result<Option<User>, AppError> {
        self.get_user_by_email(&request.email).await
    }

    async fn signup(&self, request: SignupUserRepositoryRequest) -> Result<User, AppError> {
        let user = query_file_as!(
            User,
            "./src/app/infrastructure/queries/users/insert.sql",
            request.username,
            request.email,
            request.hashed_password
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }
}

impl UsersRepositoryImpl {
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = query_as!(
            User,
            r#"
        select *
        from users
        where email = $1
            "#,
            email,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }
}
