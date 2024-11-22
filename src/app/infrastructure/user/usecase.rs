use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

use crate::app::domain::{
    error::AppError,
    jwt_token::jwt_token::JwtAuthToken,
    user::{
        repository::UserRepository,
        usecase::{SignUpResult, UserUseCase},
    },
};

#[derive(Clone)]
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
    async fn signup(
        &self,
        username: &str,
        email: &str,
        naive_password: &str,
    ) -> Result<SignUpResult, AppError> {
        let hashed_password = hasher::hash_password(naive_password)?;
        let user = self
            .user_repository
            .signup(email, username, hashed_password)
            .await?;
        let one_day: i64 = 60 * 60 * 24;
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let token = self.jwt_auth_token.generate_token(user.id, now, one_day)?;
        Ok(SignUpResult {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            bio: user.bio,
            image: user.image,
            created_at: user.created_at,
            updated_at: user.updated_at,
            token,
        })
    }
}
