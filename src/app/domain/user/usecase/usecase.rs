use crate::app::domain::error::AppError;
use async_trait::async_trait;
use sqlx::types::time::OffsetDateTime;
use tokio::signal::unix::{Signal, SignalKind};
use uuid::Uuid;

use super::{
    requests::{SigninRequest, SignupRequest},
    responses::{SigninResponse, SignupResponse},
};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn signup(&self, request: SignupRequest) -> Result<SignupResponse, AppError>;
    async fn signin(&self, request: SigninRequest) -> Result<SigninResponse, AppError>;
}