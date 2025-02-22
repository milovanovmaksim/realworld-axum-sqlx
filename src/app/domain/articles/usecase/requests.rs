use uuid::Uuid;


///
/// Запрос на создание новой статьи.
pub struct CreateArticleRequest {
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}