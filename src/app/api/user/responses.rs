use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app::domain::user::usecase::responses::{
    self as user_usecase_response, SigninUserUsecaseResponse,
};

#[derive(Serialize, Debug, ToSchema)]
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

#[derive(Serialize, Debug, ToSchema)]
pub struct SigninUserResponse {
    user: AuthUser,
}

impl From<user_usecase_response::SigninUserUsecaseResponse> for SigninUserResponse {
    fn from(value: user_usecase_response::SigninUserUsecaseResponse) -> Self {
        SigninUserResponse {
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
