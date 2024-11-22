use crate::app::domain::user::repository::entities::User;

pub struct SignupResponse {
    pub user: AuthUser,
}

impl From<(User, String)> for SignupResponse {
    fn from((user, token): (User, String)) -> Self {
        SignupResponse {
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

pub struct SigninResponse {
    pub user: AuthUser,
}

impl From<(User, String)> for SigninResponse {
    fn from((user, token): (User, String)) -> Self {
        SigninResponse {
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
