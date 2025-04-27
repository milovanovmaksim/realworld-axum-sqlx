use serde::Serialize;
use utoipa::ToSchema;

use crate::app::domain::user::usecase::responses::UserUsecase;

///
/// Ответ api сервера. Содержит информацию о пользователе.
#[derive(Serialize, Debug, ToSchema)]
pub struct AuthenticationUserResponse {
    user: User,
}

impl From<UserUsecase> for AuthenticationUserResponse {
    fn from(value: UserUsecase) -> Self {
        AuthenticationUserResponse {
            user: User {
                email: value.user.email,
                username: value.user.username,
                bio: value.user.bio,
                image: value.user.image,
                token: value.user.token,
            },
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct User {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
