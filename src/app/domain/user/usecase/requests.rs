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
            email: value.user.email,
            naive_password: value.user.password,
        }
    }
}
