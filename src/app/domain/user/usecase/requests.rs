use crate::app::api::user::requests::{SigninUserApiRequest, SignupUserApiRequest, UpdateUserApiRequest};

/// Запрос на регистрвцию нового пользовталя.
pub struct SignupUserUsecaseRequest {
    pub username: String,
    pub email: String,
    pub naive_password: String,
}

///
/// Запрос на авторизацию пользователя.
pub struct SigninUserUsecaseRequest {
    pub email: String,
    pub naive_password: String,
}

///
/// Запрос для обновления информации о пользователе.
pub struct UpdateUserUsecaseRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl From<UpdateUserApiRequest> for UpdateUserUsecaseRequest {
    fn from(value: UpdateUserApiRequest) -> Self {
        UpdateUserUsecaseRequest {
            email: value.user.email,
            username: value.user.username,
            password: value.user.password,
            bio: value.user.bio,
            image: value.user.image,
        }
    }
}

impl From<SignupUserApiRequest> for SignupUserUsecaseRequest {
    fn from(value: SignupUserApiRequest) -> Self {
        SignupUserUsecaseRequest {
            username: value.user.username.unwrap(),
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}

impl From<SigninUserApiRequest> for SigninUserUsecaseRequest {
    fn from(value: SigninUserApiRequest) -> Self {
        SigninUserUsecaseRequest {
            email: value.user.email.unwrap(),
            naive_password: value.user.password.unwrap(),
        }
    }
}
