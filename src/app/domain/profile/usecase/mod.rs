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
    /// current_user_email - email текущего пользователя;
    /// username - username запрашиваемого профиля.
    async fn get_profile(
        &self,
        user_id: Uuid,
        username: String,
    ) -> Result<ProfileResponse, AppError>;
}
