use crate::app::domain::error::AppError;
use async_trait::async_trait;

use super::{
    requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest},
    responses::{SigninUserUsecaseResponse, SignupUserUsecaseResponse},
};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn signup(
        &self,
        request: SignupUserUsecaseRequest,
    ) -> Result<SignupUserUsecaseResponse, AppError>;
    async fn signin(
        &self,
        request: SigninUserUsecaseRequest,
    ) -> Result<SigninUserUsecaseResponse, AppError>;
}
