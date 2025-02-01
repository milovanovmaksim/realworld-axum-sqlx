pub mod responses;

use crate::app::error::AppError;
use async_trait::async_trait;
use responses::ProfileResponse;
use uuid::Uuid;

///
/// ProfileUseCase интерфейс, определяющий бизнес логику профиля.
#[async_trait]
pub trait ProfileUseCase: Sync + Send + 'static {
    ///
    /// Возвращает профиль по username.
    /// current_user_id - id текущего пользователя, прошедшего аутентификацию;
    /// username - username запрашиваемого профиля.
    async fn get_profile(
        &self,
        user_id: Option<Uuid>,
        username: String,
    ) -> Result<ProfileResponse, AppError>;

    ///
    /// Текущего пользователя делает подписчиком для followee_id.
    /// current_user_id - id текущего пользователя, прошедшего аутентификацию;
    /// username - username пользователя на которого подписываем текущего пользователя.
    async fn add_user_follow(
        &self,
        current_user_id: Uuid,
        username: String,
    ) -> Result<ProfileResponse, AppError>;

    async fn remove_user_follow(
        &self,
        username: String,
        current_user_id: Uuid,
    ) -> Result<ProfileResponse, AppError>;
}
