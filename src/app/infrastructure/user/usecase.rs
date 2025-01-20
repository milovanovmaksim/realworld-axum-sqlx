use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};

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
    infrastructure::{jwt_token::claims::Claims, utils::hasher},
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
        request: SigninUserUsecaseRequest,
    ) -> Result<SigninUserUsecaseResponse, AppError> {
        let user = self
            .user_repository
            .login(requests::SigninUserRepositoryRequest {
                email: request.email.clone(),
            })
            .await?;

        match user {
            Some(user) => {
                info!("User has been found, verifying password hash for user {:?}", request.email);

                if hasher::verify(&request.naive_password, &user.password)? {
                    let one_day: i64 = 60 * 60 * 24;
                    let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
                    
                    info!("user login successful, generating token");

                    let token = self.jwt_auth_token.generate_token(Claims::new(user.id, &user.email, now, one_day))?;
                    Ok(SigninUserUsecaseResponse::from((user, token)))
                    
                } else {
                    error!("invalid login attempt for user {:?}", request.email);

                    return Err(AppError::BadRequest(String::from(format!("password is incorrect"))));
                }
            },
            None => {
                error!("user {:?} not found", request.email);

                return Err(AppError::NotFound(String::from(format!("User with email '{}' not found", request.email.clone()))));
            }          
        }

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
        let token = self.jwt_auth_token.generate_token(Claims::new(user.id, &user.email, now, one_day))?;

        Ok(SignupUserUsecaseResponse::from((user, token)))
    }
}
