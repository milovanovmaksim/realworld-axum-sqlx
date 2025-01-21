use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};
use uuid::Uuid;

use crate::app::{
    domain::{
        error::AppError,
        jwt_token::jwt_token::JwtAuthToken,
        user::{
            repository::{self, repository::UserRepository, requests},
            usecase::{
                requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest},
                responses::AuthenticationUserUsecaseResponse,
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
    ) -> Result<AuthenticationUserUsecaseResponse, AppError> {
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
                    Ok(AuthenticationUserUsecaseResponse::from((user, token)))
                    
                } else {
                    error!("invalid login attempt for user {:?}", request.email);

                    return Err(AppError::BadRequest(String::from(format!("password is incorrect"))));
                }
            },
            None => {
                error!("user {:?} not found", request.email);

                return Err(AppError::NotFound);
            }          
        }

    }

    async fn signup(
        &self,
        request: SignupUserUsecaseRequest,
    ) -> Result<AuthenticationUserUsecaseResponse, AppError> {
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

        Ok(AuthenticationUserUsecaseResponse::from((user, token)))
    }


    async fn get_current_user(&self, user_id: Uuid) -> Result<AuthenticationUserUsecaseResponse, AppError> {
        info!("retrieving user {:?}", user_id);
        let user = self.user_repository.get_user_by_id(user_id).await?;

        match user {
            Some(user) => {
                info!("user found with email {:?}, generating new token", user.email);

                let one_day: i64 = 60 * 60 * 24;
                let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second                
                let token = self.jwt_auth_token.generate_token(Claims::new(user.id, &user.email, now, one_day))?;
                Ok(AuthenticationUserUsecaseResponse::from((user, token)))
            },
            None => {
                return Err(AppError::NotFound);
            }
        }
        
    }
}
