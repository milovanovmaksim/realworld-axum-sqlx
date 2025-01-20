use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app::domain::user::usecase::responses::{
    SigninUserUsecaseResponse, SignupUserUsecaseResponse,
};

#[derive(Serialize, Debug, ToSchema)]
pub struct SignupUserResponse {
    user: AuthUser,
}

impl From<SignupUserUsecaseResponse> for SignupUserResponse {
    fn from(value: SignupUserUsecaseResponse) -> Self {
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

impl From<SigninUserUsecaseResponse> for SigninUserResponse {
    fn from(value: SigninUserUsecaseResponse) -> Self {
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
