use uuid::Uuid;

use crate::app::domain::articles::usecase::requests::CreateArticleUsecaseRequest;

///
/// Запрос на создание новой статьи.
pub struct CreateArticleRepoRequest {
    pub user_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
}

impl From<(CreateArticleUsecaseRequest, String)> for CreateArticleRepoRequest {
    fn from((request, slug): (CreateArticleUsecaseRequest, String)) -> Self {
        Self {
            user_id: request.user_id,
            title: request.title,
            slug,
            description: request.description,
            body: request.body,
        }
    }
}
