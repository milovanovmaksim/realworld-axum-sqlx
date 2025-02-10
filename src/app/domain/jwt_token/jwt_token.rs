use crate::app::{domain::user::repository::responses::UserEntity, error::AppError};

///
/// JwtAuthToken - интерфейс для работы с JWT токеном.
pub trait JwtAuthToken: Send + Sync + 'static {
    ///
    /// Генерирует JWT токен для пользователя.
    /// user -  пользователь для которго необходимо создать JWT токен.
    fn generate_token(&self, user: &UserEntity) -> Result<String, AppError>;

    ///
    /// Возвращает email пользователя из JWT токена.
    /// token - JWT токен.
    fn get_user_email_from_token(&self, token: &str) -> Result<String, AppError>;
}
