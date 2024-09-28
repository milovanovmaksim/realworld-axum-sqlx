use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use crate::app::domain::error::AppError;

pub type Token = String;

pub trait UserUseCase {
    async fn signup(
        &self,
        username: &str,
        email: &str,
        naive_password: &str,
    ) -> Result<SignupUserResult, AppError>;
}

pub struct SignupUserResult {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub token: Token,
}
