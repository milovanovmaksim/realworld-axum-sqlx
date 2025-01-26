use crate::app::{domain::user::repository::entities::User, error::AppError};

pub trait JwtAuthToken: Send + Sync + 'static {
    fn generate_token(&self, user: &User) -> Result<String, AppError>;
    fn get_user_email_from_token(&self, token: &str) -> Result<String, AppError>;
}
