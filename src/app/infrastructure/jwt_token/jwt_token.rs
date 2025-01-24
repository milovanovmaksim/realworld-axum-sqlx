use crate::app::domain::{
    error::AppError, jwt_token::jwt_token::JwtAuthToken, user::repository::entities::User,
};

use super::{claims::Claims, settings::JwtTokenSettings};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

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
    fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let claims = Claims::new(user.id, &user.email, now, self.jwt_token_settings.offset);
        let secret_key = self.jwt_token_settings.secret_key.as_bytes();
        let jwt_token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key),
        )?;
        Ok(jwt_token)
    }

    fn get_user_id_from_token(&self, token: &str) -> Result<uuid::Uuid, AppError> {
        let decoded_token = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_token_settings.secret_key.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )?;

        Ok(decoded_token.claims.user_id)
    }
}
