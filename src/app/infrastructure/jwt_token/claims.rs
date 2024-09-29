use serde::{Deserialize, Serialize};
use uuid::Uuid;


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