use crate::app::{
    domain::user::{
        self,
        repository::{responses, Email, UserRepository},
    },
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};

use async_trait::async_trait;
use sqlx::{query_as, query_file_as};
use tracing::info;
use uuid::Uuid;


///
/// Реализует интерфейс UserRepository для работы с таблицей 'users' базы данных PostgreSQL.
pub struct UsersRepositoryImpl {
    pg_sql: PostgreSQL,
}

impl UsersRepositoryImpl {
    
    ///
    /// Основной конструктор.
    pub fn new(pg_sql: PostgreSQL) -> UsersRepositoryImpl {
        UsersRepositoryImpl { pg_sql }
    }
}

#[async_trait]
impl UserRepository for UsersRepositoryImpl {
    
    ///
    /// Создает нового пользователя.
    async fn create_user(
        &self,
        request: user::repository::requests::CreateUserRequest,
    ) -> Result<responses::UserEntity, AppError> {
        info!(
            "Creating new user {:?}/{:?}",
            request.email, request.username
        );

        let user = query_file_as!(
            responses::UserEntity,
            "./src/app/infrastructure/queries/users/insert.sql",
            request.username,
            request.email,
            request.hashed_password
        )
        .fetch_one(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }
    
    
    ///
    /// Возвращает пользователя по email.
    async fn get_user_by_email(&self, email: Email) -> Result<Option<responses::UserEntity>, AppError> {
        info!("Searching for user by email in db {:?}", email);

        let user = query_as!(
            responses::UserEntity,
            r#"select * from users where email = $1"#,
            email,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }

    ///
    /// Возвращает пользователя по id.
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<responses::UserEntity>, AppError> {
        info!("Searching for user by id in db {:?}", user_id);

        let user = query_as!(
            responses::UserEntity,
            r#"select * from users where id = $1"#,
            user_id,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }

    ///
    /// Возвращает пользователя по username.
    async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<responses::UserEntity>, AppError> {
        info!("Searching for user by username in db {:?}", username);

        let user = query_as!(
            responses::UserEntity,
            r#"select * from users where username = $1"#,
            username,
        )
        .fetch_optional(&self.pg_sql.pool())
        .await?;
        Ok(user)
    }

    ///
    /// Обновляет информацию о пользователе.
    async fn update_user(
        &self,
        request: user::repository::requests::UpdateUserRequest,
    ) -> Result<responses::UserEntity, AppError> {
        info!("Updating user");

        let user = query_file_as!(
            responses::UserEntity,
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
