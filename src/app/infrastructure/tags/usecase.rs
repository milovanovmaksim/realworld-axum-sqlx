use async_trait::async_trait;

use crate::app::domain::tags::{repository::TagsRepository, usecase::TagsUseCase};

use super::repository::TagsRepositoryImpl;

pub struct TagsUsacaseImpl {
    tags_repository: TagsRepositoryImpl,
}

impl TagsUsacaseImpl {
    pub fn new(tags_repository: TagsRepositoryImpl) -> Self {
        Self { tags_repository }
    }
}

#[async_trait]
impl TagsUseCase for TagsUsacaseImpl {
    async fn get_tags(
        &self,
        tags: Vec<String>,
    ) -> Result<Vec<String>, crate::app::error::AppError> {
        Ok(self
            .tags_repository
            .get_tags(vec![])
            .await?
            .into_iter()
            .map(|v| v.tag)
            .collect())
    }
}
