use requests::CreateArticleRequest;
use responses::ArticleResponse;

use crate::app::error::AppError;


pub mod requests;
pub mod responses;


pub trait ArticlesUsecase {
    async fn cereate_article(&self, request: CreateArticleRequest) -> Result<ArticleResponse, AppError>;
}