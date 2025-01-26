use async_trait::async_trait;
use entities::Profile;

use crate::app::domain::{error::AppError, user::repository::entities::User};

pub mod entities;

#[async_trait]
pub trait ProfileRepository: Send + Sync + 'static {
    async fn get_profile_by_name(
        &self,
        current_user: &User,
        username: &str,
    ) -> Result<Profile, AppError>;
}
