use uuid::Uuid;

use crate::app::{domain::user, error::AppError, infrastructure::utils};

///
/// Запрос на создание нового пользователя в БД.
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

///
/// Запрос на обновление информации о пользователе.
pub struct UpdateUserRequest {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: Option<String>,
    pub hashed_password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl TryFrom<(Uuid, user::usecase::requests::UpdateUserRequest)> for UpdateUserRequest {
    type Error = AppError;

    fn try_from(
        (user_id, request): (Uuid, user::usecase::requests::UpdateUserRequest),
    ) -> Result<Self, Self::Error> {
        let hashed_password = match request.password {
            Some(naive_password) => {
                let password = utils::hasher::hash_password(&naive_password)?;
                Some(password)
            }
            None => None,
        };

        Ok(UpdateUserRequest {
            id: user_id,
            email: request.email,
            username: request.username,
            hashed_password,
            bio: request.bio,
            image: request.image,
        })
    }
}
