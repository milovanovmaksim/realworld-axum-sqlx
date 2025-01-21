use uuid::Uuid;

use crate::app::{domain::error::AppError, infrastructure::jwt_token::claims::Claims};

pub trait JwtAuthToken: Send + Sync + 'static {
    fn generate_token(&self, claims: Claims) -> Result<String, AppError>;
    fn get_user_id_from_token(&self, token: String) -> Result<Uuid, AppError>;
}
