use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    user_id: Uuid,
    sub: String,
}

impl Claims {
    pub fn new(user_id: Uuid, sub: &str, now: i64, offset: i64) -> Self {
        Claims {
            exp: now,
            iat: now + offset,
            user_id,
            sub: String::from(sub),
        }
    }
}
