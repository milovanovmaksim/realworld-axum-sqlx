use async_trait::async_trait;
use entities::TagEntity;

use crate::app::error::AppError;

pub mod entities;

#[async_trait]
pub trait TagsRepository {
    async fn get_tags(&self, tags: Vec<String>) -> Result<Vec<TagEntity>, AppError>;
}
