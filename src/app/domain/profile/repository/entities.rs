use chrono::{DateTime, Utc};
use uuid::Uuid;

///
/// Представляет отдельную запись в БД.
#[derive(Debug)]
pub struct UserFollow {
    pub id: Uuid,
    pub followee_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: DateTime<Utc>,
}
