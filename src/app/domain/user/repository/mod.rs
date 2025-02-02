pub mod entities;
pub mod requests;

use async_trait::async_trait;
use entities::User;
use requests::{SigninUserRequest, SignupUserRequest, UpdateUserRequest};
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
    async fn signup(&self, request: SignupUserRequest) -> Result<User, AppError>;
    async fn login(&self, request: SigninUserRequest) -> Result<Option<User>, AppError>;

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
    async fn update_user(&self, request: UpdateUserRequest) -> Result<User, AppError>;
}
