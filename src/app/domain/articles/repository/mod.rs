use std::any::Any;

use async_trait::async_trait;
use entities::Article;
use requests::CreateArticleRepoRequest;

use crate::app::error::AppError;

pub mod entities;
pub mod requests;

///
/// Интерфейс, определяющий набор методов для работы с БД.
#[async_trait]
pub trait ArticlesRepository {
    ///
    /// Добавляет новую статью в БД.
    async fn create_article(
        &self,
        ctx: Option<&mut (dyn Any + Send + Sync)>,
        article: CreateArticleRepoRequest,
    ) -> Result<Article, AppError>;
}
