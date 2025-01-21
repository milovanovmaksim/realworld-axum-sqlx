use crate::app::domain::error::AppError;
use async_trait::async_trait;
use uuid::Uuid;

use super::{
    requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest},
    responses:: AuthenticationUserUsecaseResponse
};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn signup(
        &self,
        request: SignupUserUsecaseRequest,
    ) -> Result<AuthenticationUserUsecaseResponse, AppError>;
    async fn login(
        &self,
        request: SigninUserUsecaseRequest,
    ) -> Result<AuthenticationUserUsecaseResponse, AppError>;
    async fn get_current_user(&self, user_id: Uuid) -> Result<AuthenticationUserUsecaseResponse, AppError>;
}
