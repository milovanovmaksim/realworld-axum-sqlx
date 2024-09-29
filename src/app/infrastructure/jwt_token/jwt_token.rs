use crate::app::domain::{
    error::AppError,
    jwt_token::jwt_token::JwtAuthToken,
};

use super::{claims::Claims, settings::JwtTokenSettings};
use jsonwebtoken::{EncodingKey, Header};
use uuid::Uuid;

pub struct JwtAuthTokenImpl {
    jwt_token_settings: JwtTokenSettings,
}

impl JwtAuthTokenImpl {
    pub fn new(jwt_token_settings: JwtTokenSettings) -> Self {
        Self { jwt_token_settings }
    }
}

impl JwtAuthToken for JwtAuthTokenImpl {
    fn generate_token(&self, user_id: Uuid, now: i64, offset: i64) -> Result<String, AppError> {
        let claims = Claims::new(user_id, now, offset);
        let secret_key = self.jwt_token_settings.secret_key.as_bytes();
        let jwt_token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key),
        )?;
        Ok(jwt_token)
    }
}
