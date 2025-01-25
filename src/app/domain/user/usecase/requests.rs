use crate::app::api::user::requests as user_api_requests;

pub struct SignupUserRequest {
    pub username: String,
    pub email: String,
    pub naive_password: String,
}

pub struct SigninUserRequest {
    pub email: String,
    pub naive_password: String,
}

pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl From<user_api_requests::UpdateUserRequest> for UpdateUserRequest {
    fn from(value: user_api_requests::UpdateUserRequest) -> Self {
        UpdateUserRequest {
            email: value.user.email,
            username: value.user.username,
            password: value.user.password,
            bio: value.user.bio,
            image: value.user.image,
        }
    }
}

impl From<user_api_requests::SignupUserRequest> for SignupUserRequest {
    fn from(value: user_api_requests::SignupUserRequest) -> Self {
        SignupUserRequest {
            username: value.user.username.unwrap(),
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}

impl From<user_api_requests::SigninUserRequest> for SigninUserRequest {
    fn from(value: user_api_requests::SigninUserRequest) -> Self {
        SigninUserRequest {
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}
