use chrono::{DateTime, Utc};
use uuid::Uuid;


///
/// Article содержит информацию о статье.
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
