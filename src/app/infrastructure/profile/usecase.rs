use std::sync::Arc;

use async_trait::async_trait;

use crate::app::{
    domain::{
        profile::{repository::ProfileRepository, usecase::{responses::ProfileResponse, ProfileUseCase}},
        user::repository::{Email, UserRepository},
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
        current_user_email: Email,
        username: String,
    ) -> Result<ProfileResponse, AppError> {
        match self.user_repository.get_user_by_email(current_user_email.clone()).await? {
            Some(user) => {
                tracing::info!("Current user with email '{}' found.", current_user_email.clone());

                match self.user_repository.get_user_by_username(username.clone()).await? {
                    Some(profile) => {
                        tracing::info!("Profile with username '{}' found", username);

                        let following = self.profile_repository.user_following(user.id, profile.id).await?;
                        return Ok(
                            ProfileResponse::from((following, profile))
                        );
                    },
                    None => {
                        tracing::error!("Profile with username '{}' not found.", username.clone());
                        return Err(AppError::NotFound(format!("Profile with username '{}' not found.", username)));
                    }
                }
            }
            None => {
                tracing::error!("Failed authentication, user with email '{}' not found", current_user_email);
                return Err(AppError::Unauthorized(String::from("Authentication is required to access this resource.")));
            }
        }
    }
}
