use crate::app::constants::env_key;
use dotenv::dotenv;
use std::env;

pub struct JwtTokenSettings {
    pub secret_key: String,
}

impl JwtTokenSettings {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn from_env() -> Self {
        dotenv().ok();
        let secret_key = env::var(env_key::SECRET_KEY).expect("SECRET_KEY must be set");
        Self { secret_key }
    }
}
