use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;

use crate::app::{
    domain::{
        error::AppError,
        jwt_token::jwt_token::JwtAuthToken,
        user::{
            repository::{self, repository::UserRepository, requests},
            usecase::{
                requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest},
                responses::{SigninUserUsecaseResponse, SignupUserUsecaseResponse},
                usecase::UserUseCase,
            },
        },
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
    async fn signin(
        &self,
        request: SigninUserUsecaseRequest,
    ) -> Result<SigninUserUsecaseResponse, AppError> {
        let user = self
            .user_repository
            .signin(requests::SigninUserRepositoryRequest {
                email: request.email,
            })
            .await?;

        hasher::verify(&request.naive_password, &user.password)?;

        let one_day: i64 = 60 * 60 * 24;
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let token = self.jwt_auth_token.generate_token(user.id, now, one_day)?;
        Ok(SigninUserUsecaseResponse::from((user, token)))
    }

    async fn signup(
        &self,
        request: SignupUserUsecaseRequest,
    ) -> Result<SignupUserUsecaseResponse, AppError> {
        let hashed_password = hasher::hash_password(&request.naive_password)?;

        let user = self
            .user_repository
            .signup(repository::requests::SignupUserRepositoryRequest {
                username: request.username,
                email: request.email,
                hashed_password,
            })
            .await?;

        let one_day: i64 = 60 * 60 * 24;
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let token = self.jwt_auth_token.generate_token(user.id, now, one_day)?;

        Ok(SignupUserUsecaseResponse::from((user, token)))
    }
}
