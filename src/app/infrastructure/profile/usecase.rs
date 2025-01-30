use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::app::{
    domain::{
        profile::{
            repository::ProfileRepository,
            usecase::{responses::ProfileResponse, ProfileUseCase},
        },
        user::repository::UserRepository,
    },
    error::AppError,
};

pub struct ProfileUseCaseImpl {
    pub user_repository: Arc<dyn UserRepository>,
    pub profile_repository: Arc<dyn ProfileRepository>,
}

#[async_trait]
impl ProfileUseCase for ProfileUseCaseImpl {
    async fn get_profile(
        &self,
        user_id: Option<Uuid>,
        username: &str,
    ) -> Result<ProfileResponse, AppError> {
        match self.user_repository.get_user_by_username(username).await? {
            Some(profile) => {
                tracing::info!("Profile with username '{}' found", username);

                let following = self
                    .profile_repository
                    .user_following(user_id, profile.id)
                    .await?;
                Ok(ProfileResponse::from((following, profile)))
            }
            None => {
                tracing::error!("Profile with username '{}' not found.", username);
                Err(AppError::NotFound(format!(
                    "Profile with username '{}' not found.",
                    username
                )))
            }
        }
    }
}
