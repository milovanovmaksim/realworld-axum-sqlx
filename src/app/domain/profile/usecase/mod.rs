pub mod responses;

use crate::app::{domain::user::repository::Email, error::AppError};
use async_trait::async_trait;
use responses::ProfileResponse;

///
/// ProfileUseCase интерфейс, определяющий бизнес логику профиля.
#[async_trait]
pub trait ProfileUseCase: Sync + Send + 'static {
    ///
    /// Возвращает профиль по username.
    /// current_user - email текущего пользователя, прошедшего аутентификацию;
    /// username - username запрашиваемого профиля.
    async fn get_profile(
        &self,
        current_user_email: Email,
        username: String,
    ) -> Result<ProfileResponse, AppError>;
}
