use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct TagEntity {
    pub id: Uuid,
    pub tag: String,
    pub created_at: DateTime<Utc>,
}

impl From<TagEntity> for String {
    fn from(entity: TagEntity) -> Self {
        entity.tag
    }
}
