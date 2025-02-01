pub mod entities;

use async_trait::async_trait;
use uuid::Uuid;

use crate::app::error::AppError;

///
/// ProfileRepository интерфейс, определяющий набор методов для работы с БД таблицы 'user_follows'.
#[async_trait]
pub trait ProfileRepository: Send + Sync + 'static {
    ///
    /// Возвращает true, если текущий пользователь(curretn_user_id) является подписчиком пользователя с followee_user_id.
    /// current_user_id - id текущего пользователя, прошедшего аутентификацию в системе;
    /// followee_user_id - id пользователя на которого возможно подписан текущий пользователь.
    async fn is_follower(
        &self,
        current_user_id: Uuid,
        followee_user_id: Uuid,
    ) -> Result<bool, AppError>;
}
