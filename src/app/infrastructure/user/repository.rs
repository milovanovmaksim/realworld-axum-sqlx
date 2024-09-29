use crate::app::{
    domain::{
        error::AppError,
        user::{entity::User, repository::UserRepository},
    },
    infrastructure::{pgsql::db::PostgreSQL, utils::hasher},
};
use async_trait::async_trait;
use sqlx::query_file_as;

pub struct UsersRepositoryImpl {
    pg_sql: PostgreSQL,
}

#[async_trait]
impl UserRepository for UsersRepositoryImpl {
    async fn signup(
        &self,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<User, AppError> {
        let hashed_password = hasher::hash_password(naive_password)?;
        let user = query_file_as!(
            User,
            "./src/app/infrastructure/queries/users/insert.sql",
            username,
            email,
            hashed_password
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }
}
