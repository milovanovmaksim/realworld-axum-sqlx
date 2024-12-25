use crate::app::{
    domain::{
        error::AppError,
        user::repository::{
            entities::User,
            repository::UserRepository,
            requests::{SigninRequest, SignupRequest},
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
    async fn signin(&self, request: SigninRequest) -> Result<User, AppError> {
        self.get_user_by_email(&request.email).await
    }

    async fn signup(&self, request: SignupRequest) -> Result<User, AppError> {
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
    async fn get_user_by_email(&self, email: &str) -> Result<User, AppError> {
        let user = query_as!(
            User,
            r#"
        select *
        from users
        where email = $1
            "#,
            email,
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }
}
