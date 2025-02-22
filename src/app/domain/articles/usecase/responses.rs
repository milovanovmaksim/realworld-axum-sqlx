use chrono::{DateTime, Utc};
use uuid::Uuid;


///
/// Содержит информацию о статье.
pub struct ArticleResponse {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: Author,
}


///
/// Автор статьи.
pub struct Author {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
