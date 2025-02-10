use uuid::Uuid;


///
/// Запрос на создание новой статьи.
pub struct CreateArticleRequest {
    pub user_id: Uuid,
    pub tiyle: String,
    pub slug: String,
    pub description: String,
    pub body: String,
}
