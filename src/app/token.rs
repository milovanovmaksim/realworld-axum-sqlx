use std::env;

use jsonwebtoken::{errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::constants::env_key;

static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

fn get_secret_key() -> String {
    env::var(env_key::SECRET_KEY).expect("SECRET_KEY must be set")
}

pub struct Token {
    token: String,
}

impl Token {
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
        Ok(Token::new(token))
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
    pub fn new(user_id: Uuid, now: i64) -> Self {
        Claims {
            exp: now,
            iat: now + ONE_DAY,
            user_id,
        }
    }
}
