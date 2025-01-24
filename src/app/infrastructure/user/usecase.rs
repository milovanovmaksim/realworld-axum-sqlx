use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info};
use uuid::Uuid;

use crate::app::{
    domain::{
        error::AppError,
        jwt_token::jwt_token::JwtAuthToken,
        user::{self, repository::UserRepository, usecase::UserUseCase},
    },
    infrastructure::utils::hasher,
};

pub struct UserUseCaseImpl {
    pub jwt_auth_token: Arc<dyn JwtAuthToken>,
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserUseCaseImpl {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        jwt_auth_token: Arc<dyn JwtAuthToken>,
    ) -> Self {
        Self {
            user_repository,
            jwt_auth_token,
        }
    }
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn login(
        &self,
        request: user::usecase::requests::SigninUserRequest,
    ) -> Result<user::usecase::responses::UserUsecaseResponse, AppError> {
        let user = self
            .user_repository
            .login(user::repository::requests::SigninUserRequest {
                email: request.email.clone(),
            })
            .await?;

        match user {
            Some(user) => {
                info!(
                    "User has been found, verifying password hash for user {:?}",
                    request.email
                );

                if hasher::verify(&request.naive_password, &user.password)? {
                    info!("User login successful, generating token");

                    let token = self.jwt_auth_token.generate_token(&user)?;
                    Ok(user::usecase::responses::UserUsecaseResponse::from((
                        user, token,
                    )))
                } else {
                    error!("Invalid login attempt for user {:?}", request.email);

                    return Err(AppError::BadRequest(String::from(format!(
                        "password is incorrect"
                    ))));
                }
            }
            None => {
                error!("User {:?} not found", request.email);
                return Err(AppError::NotFound);
            }
        }
    }

    async fn signup(
        &self,
        request: user::usecase::requests::SignupUserRequest,
    ) -> Result<user::usecase::responses::UserUsecaseResponse, AppError> {
        let hashed_password = hasher::hash_password(&request.naive_password)?;

        let user = self
            .user_repository
            .signup(user::repository::requests::SignupUserRequest {
                username: request.username,
                email: request.email,
                hashed_password,
            })
            .await?;

        let token = self.jwt_auth_token.generate_token(&user)?;

        Ok(user::usecase::responses::UserUsecaseResponse::from((
            user, token,
        )))
    }

    async fn get_current_user(
        &self,
        user_id: Uuid,
    ) -> Result<user::usecase::responses::UserUsecaseResponse, AppError> {
        info!("Retrieving user by id {:?}", user_id);
        let user = self.user_repository.get_user_by_id(user_id).await?;

        match user {
            Some(user) => {
                info!(
                    "User found with email {:?}, generating new token",
                    user.email
                );

                let token = self.jwt_auth_token.generate_token(&user)?;
                Ok(user::usecase::responses::UserUsecaseResponse::from((
                    user, token,
                )))
            }
            None => {
                error!("User not found");
                return Err(AppError::NotFound);
            }
        }
    }

    async fn update_user(
        &self,
        request: user::usecase::requests::UpdateUserRequest,
    ) -> Result<user::usecase::responses::UserUsecaseResponse, AppError> {
        info!("Retrieving user by id {:?}", request.id);
        match self.user_repository.get_user_by_id(request.id).await? {
            Some(user) => {
                info!("User found with email {:?}, updating user", user.email);
                let user = self
                    .user_repository
                    .update_user(user::repository::requests::UpdateUserRequest::try_from(
                        request,
                    )?)
                    .await?;

                info!("Generating token");
                let token = self.jwt_auth_token.generate_token(&user)?;

                Ok(user::usecase::responses::UserUsecaseResponse::from((
                    user, token,
                )))
            }
            None => {
                error!("User not found");
                return Err(AppError::NotFound);
            }
        }
    }
}
