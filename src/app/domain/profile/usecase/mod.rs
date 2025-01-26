pub mod responses;

use crate::app::domain::{error::AppError, user::repository::Email};
use async_trait::async_trait;
use responses::ProfileResponse;

#[async_trait]
pub trait ProfileUseCase: Sync + Send + 'static {
    async fn get_profile(
        &self,
        current_user: Email,
        username: String,
    ) -> Result<ProfileResponse, AppError>;
}
