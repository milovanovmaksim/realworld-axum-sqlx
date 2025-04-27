use async_trait::async_trait;
use requests::CreateArticleRepoRequest;
use entities::Article;

use crate::app::error::AppError;

pub mod requests;
pub mod entities;

///
/// Интерфейс, определяющий набор методов для работы с БД.
#[async_trait]
pub trait ArticlesRepository {
    ///
    /// Добавляет новую статью в БД.
    async fn create_article(&self, article: CreateArticleRepoRequest) -> Result<Article, AppError>;
}
