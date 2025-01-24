pub mod entities;
pub mod requests;

use crate::app::domain::error::AppError;
use async_trait::async_trait;
use entities::User;
use requests::{SigninUserRequest, SignupUserRequest, UpdateUserRequest};
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn signup(&self, request: SignupUserRequest) -> Result<User, AppError>;
    async fn login(&self, request: SigninUserRequest) -> Result<Option<User>, AppError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError>;
    async fn update_user(&self, request: UpdateUserRequest) -> Result<User, AppError>;
}
