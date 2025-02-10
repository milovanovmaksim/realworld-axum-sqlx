use async_trait::async_trait;
use requests::CreateArticleRequest;
use responses::ArticleEntity;

use crate::app::error::AppError;

pub mod responses;
pub mod requests;

///
/// Интерфейс, определяющий набор методов для работы с БД.
#[async_trait]
pub trait ArticlesRepository {
    ///
    /// Добавляет новую статью в БД.
    async fn create_article(&self, article: CreateArticleRequest) -> Result<ArticleEntity, AppError>;
}
