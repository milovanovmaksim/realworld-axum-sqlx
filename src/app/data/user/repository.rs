use sqlx::query_file_as;

use crate::app::{
    data::{db::DbPool, utils::hasher},
    domain::{
        error::AppError,
        user::{entity::User, repository::UserRepository},
    },
};

#[derive(Clone)]
pub struct UsersRepositoryImpl {
    pool: DbPool,
}

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
            "./src/app/data/queries/users/insert.sql",
            username,
            email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
