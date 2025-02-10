use async_trait::async_trait;
use entities::Tag;

use crate::app::error::AppError;

pub mod entities;

///
/// TagsRepository интерфейс, определяющий набор методов для работы с БД.
#[async_trait]
pub trait TagsRepository {
    ///
    /// Возвращает тэги по имени.
    /// tags - вектор, сордержащий имена тэгов.
    async fn get_tags(&self, tags: Vec<String>) -> Result<Vec<Tag>, AppError>;

    ///
    /// Добавляет тэги в БД.
    async fn create_tags(&self, tags: Vec<String>) -> Result<Vec<Tag>, AppError>;
}
