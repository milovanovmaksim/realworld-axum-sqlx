use crate::app::domain::user::repository::entities;

type Token = String;

pub struct UserUsecaseResponse {
    pub user: User,
}

impl From<(entities::User, Token)> for UserUsecaseResponse {
    fn from((user, token): (entities::User, String)) -> Self {
        UserUsecaseResponse {
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
