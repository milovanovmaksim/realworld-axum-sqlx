use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SignupRequest {
    #[validate]
    pub user: SignupUser,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SignupUser {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,

    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,

    #[validate(required, length(min = 8))]
    pub password: Option<String>,
}
