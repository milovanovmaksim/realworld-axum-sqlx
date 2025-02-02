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
///
/// Реализует интерфейс ProfileUseCase.
/// Реализует набор методов уровня бизнес-логики профиля.
pub struct ProfileUseCaseImpl {
    pub user_repository: Arc<dyn UserRepository>,
    pub profile_repository: Arc<dyn ProfileRepository>,
}

impl ProfileUseCaseImpl {
    ///
    /// Основной конструктор.
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        profile_repository: Arc<dyn ProfileRepository>,
    ) -> Self {
        Self {
            user_repository,
            profile_repository,
        }
    }
}

#[async_trait]
impl ProfileUseCase for ProfileUseCaseImpl {
    ///
    /// Возвращает информацию о профиле по username.
    /// user_id - id текущего прользователя;
    /// username - username запрашиваемого профиля.
    async fn get_profile(
        &self,
        user_id: Option<Uuid>,
        username: String,
    ) -> Result<ProfileResponse, AppError> {
        match self
            .user_repository
            .get_user_by_username(username.clone())
            .await?
        {
            Some(profile) => {
                tracing::info!("Profile with username '{}' found.", username);
                match user_id {
                    // in the case a token is passed and validly extracted.
                    Some(user_id) => {
                        let following = self
                            .profile_repository
                            .is_follower(user_id, profile.id)
                            .await?;
                        Ok(ProfileResponse::from((following, profile)))
                    }
                    None => Ok(ProfileResponse::from((false, profile))),
                }
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

    ///
    /// Делает текущего пользователя подписчиком.
    /// current_user_id - id текущего пользователя;
    /// username - username пользователя на которого хотим подписаться.
    async fn add_user_follow(
        &self,
        current_user_id: Uuid,
        username: String,
    ) -> Result<ProfileResponse, AppError> {
        match self
            .user_repository
            .get_user_by_username(username.clone())
            .await?
        {
            Some(followee) => {
                if !self
                    .profile_repository
                    .is_follower(current_user_id, followee.id)
                    .await?
                {
                    self.profile_repository
                        .add_user_follow(current_user_id, followee.id)
                        .await?;
                }
                Ok(ProfileResponse::from((true, followee)))
            }
            None => Err(AppError::NotFound(format!(
                "Profile to follow with username '{}' not found.",
                username
            ))),
        }
    }

    ///
    /// Отписывает текущего пользователя.
    /// current_user_id - id текущего пользователя;
    /// username - username пользователя от которого хотим отписаться.
    async fn remove_user_follow(
        &self,
        username: String,
        current_user_id: Uuid,
    ) -> Result<ProfileResponse, AppError> {
        match self
            .user_repository
            .get_user_by_username(username.clone())
            .await?
        {
            Some(followee) => {
                if self
                    .profile_repository
                    .is_follower(current_user_id, followee.id)
                    .await?
                {
                    self.profile_repository
                        .remove_user_follow(current_user_id, followee.id)
                        .await?;
                }
                Ok(ProfileResponse::from((false, followee)))
            }
            None => Err(AppError::NotFound(format!(
                "Profile to unfollow with username '{}' not found.",
                username
            ))),
        }
    }
}
