pub mod requests;
pub mod responses;

use async_trait::async_trait;
use requests::{SigninUserRequest, SignupUserRequest, UpdateUserRequest};
use responses::UserUsecaseResponse;
use uuid::Uuid;

use crate::app::error::AppError;

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn signup(&self, request: SignupUserRequest) -> Result<UserUsecaseResponse, AppError>;
    async fn login(&self, request: SigninUserRequest) -> Result<UserUsecaseResponse, AppError>;
    async fn get_current_user(&self, user_id: Uuid) -> Result<UserUsecaseResponse, AppError>;
    async fn update_user(
        &self,
        request: (Uuid, UpdateUserRequest),
    ) -> Result<UserUsecaseResponse, AppError>;
}
