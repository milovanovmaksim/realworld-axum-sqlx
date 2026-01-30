pub mod entities;

use async_trait::async_trait;
use entities::UserFollow;
use uuid::Uuid;

use crate::app::error::AppError;

///
/// ProfileRepository интерфейс, определяющий набор методов для работы с БД.
#[async_trait]
pub trait ProfileRepository: Send + Sync + 'static {
    ///
    /// Возвращает true, если запись существует в БД.
    async fn is_follower(&self, user_id: Uuid, followee_id: Uuid) -> Result<bool, AppError>;

    ///
    /// Создает новую запись в БД.
    async fn add_user_follow(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> Result<UserFollow, AppError>;

    ///
    /// Удаляет запись из БД.
    async fn remove_user_follow(
        &self,
        follower_id: Uuid,
        followee_id: Uuid,
    ) -> Result<(), AppError>;
}
