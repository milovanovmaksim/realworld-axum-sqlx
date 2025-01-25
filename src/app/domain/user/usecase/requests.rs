use crate::app::api;

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

impl From<api::user::requests::UpdateUserRequest> for UpdateUserRequest {
    fn from(value: api::user::requests::UpdateUserRequest) -> Self {
        UpdateUserRequest {
            email: value.user.email,
            username: value.user.username,
            password: value.user.password,
            bio: value.user.bio,
            image: value.user.image,
        }
    }
}

impl From<api::user::requests::SignupUserRequest> for SignupUserRequest {
    fn from(value: api::user::requests::SignupUserRequest) -> Self {
        SignupUserRequest {
            username: value.user.username.unwrap(),
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}

impl From<api::user::requests::SigninUserRequest> for SigninUserRequest {
    fn from(value: api::user::requests::SigninUserRequest) -> Self {
        SigninUserRequest {
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}
