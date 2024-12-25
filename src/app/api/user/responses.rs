use serde::{Deserialize, Serialize};

use crate::app::domain::user::usecase::responses as user_usecase_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupResponse {
    user: AuthUser,
}

impl From<user_usecase_response::SignupResponse> for SignupResponse {
    fn from(value: user_usecase_response::SignupResponse) -> Self {
        SignupResponse {
            user: AuthUser {
                email: value.user.email,
                username: value.user.username,
                bio: value.user.bio,
                image: value.user.image,
                token: value.user.token,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
