use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

///
/// Содержит информацию о пользовтеле.
/// Представляет отдельную запись в БД.
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
