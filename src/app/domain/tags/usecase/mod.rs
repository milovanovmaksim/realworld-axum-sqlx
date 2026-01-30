use async_trait::async_trait;

use crate::app::error::AppError;

///
/// Интерфейс, определяющий набор методов бизнес-логики для работы с тэгами.
#[async_trait]
pub trait TagsUseCase {
    async fn get_tags(&self) -> Result<Vec<String>, AppError>;
    async fn create_article_tags(&self, tags: Vec<(i64, i64)>) -> Result<(), AppError>;
}
