use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    pub user_id: Uuid,
    pub email: String,
}

impl Claims {
    pub fn new(user_id: Uuid, email: &str, now: i64, offset: i64) -> Self {
        Claims {
            exp: now + offset,
            iat: offset,
            user_id,
            email: String::from(email),
        }
    }
}
