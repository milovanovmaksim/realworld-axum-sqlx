use jsonwebtoken::{errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::app::constants::env_key;

fn get_secret_key() -> String {
    env::var(env_key::SECRET_KEY).expect("SECRET_KEY must be set")
}

pub struct JwtAuthToken {
    token: String,
}

impl JwtAuthToken {
    fn new(token: String) -> Self {
        Self { token }
    }

    pub fn from_claims(claims: Claims) -> Result<Self, Error> {
        let binding = get_secret_key();
        let secret_key = binding.as_bytes();
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key),
        )?;
        Ok(Self::new(token))
    }

    pub fn token(&self) -> String {
        self.token.clone()
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
