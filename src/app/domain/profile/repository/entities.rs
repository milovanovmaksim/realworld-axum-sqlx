use chrono::{DateTime, Utc};
use uuid::Uuid;

///
/// Подписка пользователя.
#[derive(Debug)]
pub struct UserFollowEntity {
    pub id: Uuid,
    pub followee_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: DateTime<Utc>,
}
