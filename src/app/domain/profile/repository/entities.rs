use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Debug)]
pub struct UserFollow {
    pub id: Uuid,
    pub followee_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: DateTime<Utc>,
}
