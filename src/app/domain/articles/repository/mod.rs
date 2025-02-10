use async_trait::async_trait;
use entities::{ArticleEntity, CreateArticleRequest};

use crate::app::error::AppError;

pub mod entities;

///
/// Интерфейс, определяющий набор методов для работы с БД.
#[async_trait]
pub trait ArticlesRepository {
    ///
    /// Добавляет новую статью в БД.
    async fn create_article(&self, article: CreateArticleRequest) -> Result<ArticleEntity, AppError>;
}
