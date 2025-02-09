use async_trait::async_trait;
use responses::TagEntity;

use crate::app::error::AppError;

pub mod responses;

///
/// TagsRepository интерфейс, определяющий набор методов для работы с сущностью TagEntity в БД.
#[async_trait]
pub trait TagsRepository {
    ///
    /// Возвращает тэги по имени.
    /// tags - вектор, сордержащий имена тэгов.
    async fn get_tags(&self, tags: Vec<String>) -> Result<Vec<TagEntity>, AppError>;

    ///
    /// Создает новые тэги.
    async fn create_tags(&self, tags: Vec<String>) -> Result<Vec<TagEntity>, AppError>;
}
