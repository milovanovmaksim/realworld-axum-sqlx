use uuid::Uuid;

use crate::app::{domain::user::usecase::requests::UpdateUserUsecaseRequest, error::AppError, infrastructure::utils};

///
/// Запрос на создание нового пользователя в БД.
pub struct CreateUserRepoRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

///
/// Запрос на обновление информации о пользователе.
pub struct UpdateUserRepoRequest {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: Option<String>,
    pub hashed_password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl TryFrom<(Uuid, UpdateUserUsecaseRequest)> for UpdateUserRepoRequest {
    type Error = AppError;

    fn try_from(
        (user_id, request): (Uuid, UpdateUserUsecaseRequest),
    ) -> Result<Self, Self::Error> {
        let hashed_password = match request.password {
            Some(naive_password) => {
                let password = utils::hasher::hash_password(&naive_password)?;
                Some(password)
            }
            None => None,
        };

        Ok(UpdateUserRepoRequest {
            id: user_id,
            email: request.email,
            username: request.username,
            hashed_password,
            bio: request.bio,
            image: request.image,
        })
    }
}
