use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app::domain::user::usecase::responses as user_usecase_response;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SignupUserResponse {
    user: AuthUser,
}

impl From<user_usecase_response::SignupUserUsecaseResponse> for SignupUserResponse {
    fn from(value: user_usecase_response::SignupUserUsecaseResponse) -> Self {
        SignupUserResponse {
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

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AuthUser {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
