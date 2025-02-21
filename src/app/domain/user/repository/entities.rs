use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

///
/// User представляет отдельную запись в БД.
pub struct UserEntity {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
