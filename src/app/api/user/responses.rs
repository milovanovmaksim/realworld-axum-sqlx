use serde::Serialize;
use utoipa::ToSchema;

use crate::app::domain::user::usecase::responses::UserUsecaseResponse;


///
/// Ответ api сервера.
#[derive(Serialize, Debug, ToSchema)]
pub struct AuthenticationUserResponse {
    user: User,
}

impl From<UserUsecaseResponse> for AuthenticationUserResponse {
    fn from(value: UserUsecaseResponse) -> Self {
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
