use chrono::{DateTime, Utc};
use uuid::Uuid;


///
/// Article содержит информацию о статье.
pub struct ArticleEntity {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


///
/// Запрос на создание новой статьи.
pub struct CreateArticleRequest {
    pub user_id: Uuid,
    pub tiyle: String,
    pub slug: String,
    pub description: String,
    pub body: String,
}
