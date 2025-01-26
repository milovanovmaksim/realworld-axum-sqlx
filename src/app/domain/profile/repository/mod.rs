pub mod entities;

use async_trait::async_trait;
use uuid::Uuid;

use crate::app::error::AppError;

///
/// ProfileRepository интерфейс, определяющий набор методов для работы с БД таблицы 'user_follows'.
#[async_trait]
pub trait ProfileRepository: Send + Sync + 'static {
    ///
    /// Возвращает true, если пользователь с following_user_id подписан
    /// на пользователя с current_user_id.
    /// current_user_id - текущий пользователь залогининый в системе;
    /// following_user_id - пользователь, которого
    /// хотим проверить подписан ли он на текущего пользователя.
    async fn user_following(
        &self,
        current_user_id: Uuid,
        following_user_id: Uuid,
    ) -> Result<bool, AppError>;
}
