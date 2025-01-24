use uuid::Uuid;

use crate::app::api::user::requests as user_api_requests;

pub struct SignupUserUsecaseRequest {
    pub username: String,
    pub email: String,
    pub naive_password: String,
}

pub struct SigninUserUsecaseRequest {
    pub email: String,
    pub naive_password: String,
}

pub struct UpdateUserRequest {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl From<(Uuid, user_api_requests::UpdateUserRequest)> for UpdateUserRequest {
    fn from((id, value): (Uuid, user_api_requests::UpdateUserRequest)) -> Self {
        UpdateUserRequest {
            id,
            email: value.user.email,
            username: value.user.username,
            password: value.user.password,
            bio: value.user.bio,
            image: value.user.image,
        }
    }
}

impl From<user_api_requests::SignupUserRequest> for SignupUserUsecaseRequest {
    fn from(value: user_api_requests::SignupUserRequest) -> Self {
        SignupUserUsecaseRequest {
            username: value.user.username.unwrap(),
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}

impl From<user_api_requests::SigninUserRequest> for SigninUserUsecaseRequest {
    fn from(value: user_api_requests::SigninUserRequest) -> Self {
        SigninUserUsecaseRequest {
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}
