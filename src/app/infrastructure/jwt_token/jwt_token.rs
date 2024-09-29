use super::settings::JwtTokenSettings;
use jsonwebtoken::{errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct JwtAuthToken {
    jwt_token: String,
}

impl JwtAuthToken {
    pub fn new(jwt_token: String) -> Self {
        Self { jwt_token }
    }

    pub fn generate_token(
        claims: Claims,
        jwt_token_settings: &JwtTokenSettings,
    ) -> Result<Self, Error> {
        let secret_key = jwt_token_settings.secret_key.as_bytes();
        let jwt_token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key),
        )?;
        Ok(Self::new(jwt_token))
    }

    pub fn token(&self) -> String {
        self.jwt_token.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    pub user_id: Uuid,
}

impl Claims {
    pub fn new(user_id: Uuid, now: i64, offset: i64) -> Self {
        Claims {
            exp: now,
            iat: now + offset,
            user_id,
        }
    }
}
