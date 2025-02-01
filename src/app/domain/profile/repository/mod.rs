pub mod entities;

use async_trait::async_trait;
use entities::UserFollow;
use uuid::Uuid;

use crate::app::error::AppError;

///
/// ProfileRepository интерфейс, определяющий набор методов для работы с БД таблицы 'user_follows'.
#[async_trait]
pub trait ProfileRepository: Send + Sync + 'static {
    ///
    /// Возвращает true, если пользователь curretn_user_id является подписчиком пользователя followee_id.
    /// current_user_id - id пользователя;
    /// followee_id - id пользователя на которого возможно подписан текущий пользователь.
    async fn is_follower(&self, current_user_id: Uuid, followee_id: Uuid)
        -> Result<bool, AppError>;

    ///
    /// Создает новую запись в таблицу user_follow.
    /// follower_id - id пользователя(подписчик);
    /// followee_id - id пользователя на которого подписываем follower_id.
    async fn add_user_follow(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> Result<UserFollow, AppError>;
}
