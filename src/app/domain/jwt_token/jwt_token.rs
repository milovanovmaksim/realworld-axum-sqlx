use async_trait::async_trait;
use uuid::Uuid;

use crate::app::domain::error::AppError;

#[async_trait]
pub trait JwtAuthToken: Send + Sync + 'static {
    fn generate_token(&self, user_id: Uuid, now: i64, offset: i64) -> Result<String, AppError>;
}
