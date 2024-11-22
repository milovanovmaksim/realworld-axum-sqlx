use crate::app::domain::error::AppError;
use async_trait::async_trait;

use super::{
    entities::User,
    requests::{SigninRequest, SignupRequest},
};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn signup(&self, request: SignupRequest) -> Result<User, AppError>;
    async fn signin(&self, request: SigninRequest) -> Result<User, AppError>;
}
