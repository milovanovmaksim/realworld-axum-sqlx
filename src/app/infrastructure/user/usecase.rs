use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

use super::token::{Claims, JwtAuthToken};
use crate::app::domain::{
    error::AppError,
    user::{
        repository::UserRepository,
        usecase::{SignUpResult, UserUseCase},
    },
};

#[derive(Clone)]
pub struct UserUseCaseImpl {
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
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
        let token = JwtAuthToken::from_claims(Claims::new(user.id, now, one_day))?.token();
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
