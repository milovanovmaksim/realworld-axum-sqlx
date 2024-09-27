use sqlx::query_file_as;

use crate::app::{
    core::{
        error::AppError,
        user::{entity::User, repository::UsersRepository},
    },
    infrastructure::{db::DbPool, utils::hasher},
};

pub struct UsersRepositoryImpl {
    pool: DbPool,
}

impl UsersRepository for UsersRepositoryImpl {
    async fn signup(
        &self,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<User, AppError> {
        let hashed_password = hasher::hash_password(naive_password)?;
        query_file_as!(
            User,
            "./src/app/infrastructure/queries/users/insert.sql",
            username,
            email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await
    }
}
