use crate::app::domain::user::repository::entities::User;

type Token = String;

pub struct SignupUserUsecaseResponse {
    pub user: AuthUser,
}

impl From<(User, Token)> for SignupUserUsecaseResponse {
    fn from((user, token): (User, String)) -> Self {
        SignupUserUsecaseResponse {
            user: AuthUser {
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
                token,
            },
        }
    }
}

pub struct SigninUserUsecaseResponse {
    pub user: AuthUser,
}

impl From<(User, Token)> for SigninUserUsecaseResponse {
    fn from((user, token): (User, String)) -> Self {
        SigninUserUsecaseResponse {
            user: AuthUser {
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
                token,
            },
        }
    }
}

pub struct AuthUser {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
