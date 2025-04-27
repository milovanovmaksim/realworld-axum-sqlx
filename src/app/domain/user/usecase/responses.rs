use crate::app::domain::user::repository::entities;

type Token = String;

///
/// Ответ, возвращаемый слоем бизнес логики пользователя.
/// Содержит информацию о пользователе.
pub struct UserUsecase {
    pub user: User,
}

impl From<(entities::User, Token)> for UserUsecase {
    fn from((user, token): (entities::User, String)) -> Self {
        UserUsecase {
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
