use crate::app::domain::error::AppError;
use async_trait::async_trait;
use uuid::Uuid;

use super::{
    requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest, UpdateUserRequest},
    responses:: UserUsecaseResponse
};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn signup(
        &self,
        request: SignupUserUsecaseRequest,
    ) -> Result<UserUsecaseResponse, AppError>;
    async fn login(
        &self,
        request: SigninUserUsecaseRequest,
    ) -> Result<UserUsecaseResponse, AppError>;
    async fn get_current_user(&self, user_id: Uuid) -> Result<UserUsecaseResponse, AppError>;
    async fn update_user(&self, request: UpdateUserRequest) -> Result<UserUsecaseResponse, AppError>;
}
