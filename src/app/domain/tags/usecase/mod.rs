use async_trait::async_trait;

use crate::app::error::AppError;

#[async_trait]
pub trait TagsUseCase {
    async fn get_tags(&self) -> Result<Vec<String>, AppError>;
}
