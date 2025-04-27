use requests::CreateArticleUsecaseRequest;
use responses::ArticleUsecase;

use crate::app::error::AppError;


pub mod requests;
pub mod responses;


///
/// Интерфейс, определяющий набор методов бизнес-логики статьи(artcile).
pub trait ArticlesUsecase {
    async fn cereate_article(&self, request: CreateArticleUsecaseRequest) -> Result<ArticleUsecase, AppError>;
}