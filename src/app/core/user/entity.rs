use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}
