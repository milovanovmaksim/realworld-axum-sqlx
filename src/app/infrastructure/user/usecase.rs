use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

use crate::app::{
    domain::{
        error::AppError,
        user::{
            repository::UserRepository,
            usecase::{SignUpResult, UserUseCase},
        },
    },
    infrastructure::jwt_token::{
        jwt_token::{Claims, JwtAuthToken},
        settings::JwtTokenSettings,
    },
};

#[derive(Clone)]
pub struct UserUseCaseImpl {
    pub jwt_token_settings: JwtTokenSettings,
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserUseCaseImpl {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        jwt_token_settings: JwtTokenSettings,
    ) -> Self {
        Self {
            jwt_token_settings,
            user_repository,
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
        let user = self
            .user_repository
            .signup(email, username, naive_password)
            .await?;
        let one_day: i64 = 60 * 60 * 24;
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let token = JwtAuthToken::generate_token(
            Claims::new(user.id, now, one_day),
            &self.jwt_token_settings,
        )?
        .token();
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
