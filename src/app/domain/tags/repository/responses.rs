use chrono::{DateTime, Utc};
use uuid::Uuid;

///
/// Тэг статьи. Содержит информацию о тэге.
/// Представляет отдельную запись в БД.
#[derive(Debug)]
pub struct Tag {
    pub id: Uuid,
    pub tag: String,
    pub created_at: DateTime<Utc>,
}

impl From<Tag> for String {
    fn from(entity: Tag) -> Self {
        entity.tag
    }
}
