use std::sync::Arc;

use async_trait::async_trait;

use crate::app::domain::tags::{repository::TagsRepository, usecase::TagsUseCase};

use super::repository::TagsRepositoryImpl;

///
/// Реализует набор методов уровня бизнес-логики тэга.
pub struct TagsUsacaseImpl {
    tags_repository: Arc<TagsRepositoryImpl>,
}

impl TagsUsacaseImpl {
    pub fn new(tags_repository: Arc<TagsRepositoryImpl>) -> Self {
        Self { tags_repository }
    }
}

#[async_trait]
impl TagsUseCase for TagsUsacaseImpl {
    ///
    /// Возвращает список тэгов по имени.
    async fn get_tags(&self) -> Result<Vec<String>, crate::app::error::AppError> {
        Ok(self
            .tags_repository
            .get_tags(vec![])
            .await?
            .into_iter()
            .map(|v| v.tag)
            .collect())
    }
}
