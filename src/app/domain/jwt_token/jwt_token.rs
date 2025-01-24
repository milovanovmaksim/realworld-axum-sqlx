use uuid::Uuid;

use crate::app::domain::{error::AppError, user::repository::entities::User};

pub trait JwtAuthToken: Send + Sync + 'static {
    fn generate_token(&self, user: &User) -> Result<String, AppError>;
    fn get_user_id_from_token(&self, token: &str) -> Result<Uuid, AppError>;
}
