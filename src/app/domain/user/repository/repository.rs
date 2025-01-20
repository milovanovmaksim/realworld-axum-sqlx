use crate::app::domain::error::AppError;
use async_trait::async_trait;

use super::{
    entities::User,
    requests::{SigninUserRepositoryRequest, SignupUserRepositoryRequest},
};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn signup(&self, request: SignupUserRepositoryRequest) -> Result<User, AppError>;
    async fn login(&self, request: SigninUserRepositoryRequest) -> Result<User, AppError>;
}
