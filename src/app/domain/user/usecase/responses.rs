use crate::app::domain::user::repository::entities;

type Token = String;

pub struct AuthenticationUserUsecaseResponse {
    pub user: User,
}

impl From<(entities::User, Token)> for AuthenticationUserUsecaseResponse {
    fn from((user, token): (entities::User, String)) -> Self {
        AuthenticationUserUsecaseResponse {
            user: User {
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
                token,
            },
        }
    }
}

pub struct User {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
