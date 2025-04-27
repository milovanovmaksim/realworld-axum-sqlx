use uuid::Uuid;


///
/// Запрос на создание новой статьи.
pub struct CreateArticleRepoRequest {
    pub user_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
}
