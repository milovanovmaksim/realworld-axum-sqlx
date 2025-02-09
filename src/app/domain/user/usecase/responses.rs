use crate::app::domain::user::repository::entities;

type Token = String;

///
/// Ответ, возвращаемый слоем бизнес логики пользователя.
/// Содержит информацию о пользователе.
pub struct UserUsecaseResponse {
    pub user: User,
}

impl From<(entities::UserEntity, Token)> for UserUsecaseResponse {
    fn from((user, token): (entities::UserEntity, String)) -> Self {
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
