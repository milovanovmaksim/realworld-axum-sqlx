use crate::app::domain::{error::AppError, jwt_token::jwt_token::JwtAuthToken};

use super::{claims::Claims, settings::JwtTokenSettings};
use jsonwebtoken::{EncodingKey, Header};

#[derive(Clone)]
pub struct JwtAuthTokenImpl {
    jwt_token_settings: JwtTokenSettings,
}

impl JwtAuthTokenImpl {
    pub fn new(jwt_token_settings: JwtTokenSettings) -> Self {
        Self { jwt_token_settings }
    }
}

impl JwtAuthToken for JwtAuthTokenImpl {
    fn generate_token(&self, claims: Claims) -> Result<String, AppError> {
        let secret_key = self.jwt_token_settings.secret_key.as_bytes();
        let jwt_token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key),
        )?;
        Ok(jwt_token)
    }
}
