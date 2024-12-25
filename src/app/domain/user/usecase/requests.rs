use crate::app::api::user::requests as user_api_requests;

pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub naive_password: String,
}

pub struct SigninRequest {
    pub email: String,
    pub naive_password: String,
}

impl From<user_api_requests::SignupRequest> for SignupRequest {
    fn from(value: user_api_requests::SignupRequest) -> Self {
        SignupRequest {
            username: value.user.username.unwrap(),
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}
