pub mod entities;
pub mod requests;

use async_trait::async_trait;
use entities::User;
use requests::{SigninUserRequest, SignupUserRequest, UpdateUserRequest};

use crate::app::error::AppError;

pub type Email = String;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn signup(&self, request: SignupUserRequest) -> Result<User, AppError>;
    async fn login(&self, request: SigninUserRequest) -> Result<Option<User>, AppError>;
    async fn get_user_by_email(&self, email: Email) -> Result<Option<User>, AppError>;
    async fn get_user_by_username(&self, username: String) -> Result<Option<User>, AppError>;
    async fn update_user(&self, request: UpdateUserRequest) -> Result<User, AppError>;
}
