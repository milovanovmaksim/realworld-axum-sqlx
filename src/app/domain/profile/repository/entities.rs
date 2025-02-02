use chrono::{DateTime, Utc};
use uuid::Uuid;

///
/// Содержит информацию о профиле.
#[derive(Debug)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

///
/// Представляет отдельную запись в БД.
/// followee_id - id пользователя на которого подписан пользователь с follower_id;
/// follower_id - id  подписчика.
#[derive(Debug)]
pub struct UserFollow {
    pub id: Uuid,
    pub followee_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: DateTime<Utc>,
}
