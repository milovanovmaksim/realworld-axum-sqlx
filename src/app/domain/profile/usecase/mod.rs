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
        username: &str,
    ) -> Result<ProfileResponse, AppError>;
}
