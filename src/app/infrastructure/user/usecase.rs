use async_trait::async_trait;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

use crate::app::{
    domain::{
        jwt_token::jwt_token::JwtAuthToken,
        user::{repository::{UserRepository, requests::{CreateUserRepoRequest, UpdateUserRepoRequest}}, usecase::{UserUseCase, requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest, UpdateUserUsecaseRequest}, responses::UserUsecaseResponse}},
    },
    error::AppError,
    infrastructure::utils::hasher,
};

///
/// Реализует набор методов уровня бизнес-логики пользователя.
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
    ///
    /// Авторизация пользователя.
    async fn login(
        &self,
        request: SigninUserUsecaseRequest,
    ) -> Result<UserUsecaseResponse, AppError> {
        let user = self
            .user_repository
            .get_user_by_email(request.email.clone())
            .await?;

        match user {
            Some(user) => {
                info!(
                    "User has been found, verifying password hash for user {:?}.",
                    request.email
                );

                if hasher::verify(&request.naive_password, &user.password)? {
                    info!("User login successful, generating token.");
                    let token = self.jwt_auth_token.generate_token(&user)?;
                    Ok(UserUsecaseResponse::from((
                        user, token,
                    )))
                } else {
                    error!(
                        "Invalid login attempt for user {:?}, password is incorrect.",
                        request.email
                    );

                    Err(AppError::BadRequest(format!("password is incorrect")))
                }
            }
            None => {
                error!("User with email '{}' not found.", request.email);
                Err(AppError::NotFound(format!(
                    "User with email '{}' not found.",
                    request.email
                )))
            }
        }
    }

    ///
    /// Регистрация нового пользователя.
    async fn signup(
        &self,
        request: SignupUserUsecaseRequest,
    ) -> Result<UserUsecaseResponse, AppError> {
        info!("Creating password hash for user {:?}", request.email);
        let hashed_password = hasher::hash_password(&request.naive_password)?;

        info!(
            "Password hashed successfully, creating user {:?}",
            request.email
        );
        let user = self
            .user_repository
            .create_user(CreateUserRepoRequest {
                username: request.username,
                email: request.email,
                hashed_password,
            })
            .await?;

        info!("User successfully created, generating token");
        let token = self.jwt_auth_token.generate_token(&user)?;

        Ok(UserUsecaseResponse::from((
            user, token,
        )))
    }

    ///
    /// Возвращает текущего пользователя по id.
    async fn get_current_user(
        &self,
        user_id: Uuid,
    ) -> Result<UserUsecaseResponse, AppError> {
        info!("Retrieving user by id {:?}", user_id);

        // a token is passed and validly extracted, user with user_id exists.
        let user = self.user_repository.get_user_by_id(user_id).await?.unwrap();

        info!(
            "User found with email {:?}, generating new token.",
            user.email
        );
        let token = self.jwt_auth_token.generate_token(&user)?;

        Ok(UserUsecaseResponse::from((
            user, token,
        )))
    }

    ///
    /// Обновляет информацию о пользователе.
    async fn update_user(
        &self,
        (user_id, request): (Uuid, UpdateUserUsecaseRequest),
    ) -> Result<UserUsecaseResponse, AppError> {
        info!("Update user {:?}", user_id);
        let user = self
            .user_repository
            .update_user(UpdateUserRepoRequest::try_from((
                user_id, request,
            ))?)
            .await?;

        info!("User {:?} updated, generating a new token", user_id);
        let token = self.jwt_auth_token.generate_token(&user)?;

        Ok(UserUsecaseResponse::from((
            user, token,
        )))
    }
}
