pub mod responses;

use crate::app::error::AppError;
use async_trait::async_trait;
use responses::ProfileResponse;
use uuid::Uuid;

///
/// ProfileUseCase интерфейс, определяющий набор методов бизнес логики профиля.
#[async_trait]
pub trait ProfileUseCase: Sync + Send + 'static {
    ///
    /// Возвращает информацию о профиле по username.
    /// current_user_id - id текущего пользователя, прошедшего аутентификацию;
    /// username - username запрашиваемого профиля.
    async fn get_profile(
        &self,
        user_id: Option<Uuid>,
        username: String,
    ) -> Result<ProfileResponse, AppError>;

    ///
    /// Делает текущего пользователя подписчиком.
    /// current_user_id - id текущего пользователя;
    /// username - username пользователя на которого хотим подписаться.
    async fn add_user_follow(
        &self,
        current_user_id: Uuid,
        username: String,
    ) -> Result<ProfileResponse, AppError>;

    ///
    /// Отписывает текущего пользователя.
    /// current_user_id - id текущего пользователя;
    /// username - username пользователя от которого хотим отписаться.
    async fn remove_user_follow(
        &self,
        username: String,
        current_user_id: Uuid,
    ) -> Result<ProfileResponse, AppError>;
}
