pub mod entities;
pub mod requests;

use async_trait::async_trait;
use entities::User;
use requests::{CreateUserRepoRequest, UpdateUserRepoRequest};
use uuid::Uuid;

use crate::app::error::AppError;

pub type Email = String;

///
/// UserRepository интерфейс, определяющий набор CRUD методов для работы с БД.
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    ///
    /// Создает нового пользователя в БД.
    /// request - запрос на создание нового пользователя в БД.
    async fn create_user(&self, request: CreateUserRepoRequest) -> Result<User, AppError>;

    ///
    /// Возвращает пользователя по email.
    async fn get_user_by_email(&self, email: Email) -> Result<Option<User>, AppError>;

    ///
    /// Возвращает пользователя по id.
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError>;

    ///
    /// Возвращает пользователя по username.
    async fn get_user_by_username(&self, username: String) -> Result<Option<User>, AppError>;

    ///
    /// Обновляет информацию о пльзователе.
    async fn update_user(&self, request: UpdateUserRepoRequest) -> Result<User, AppError>;
}
