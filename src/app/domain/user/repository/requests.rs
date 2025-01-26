use uuid::Uuid;

use crate::app::{domain::user, error::AppError, infrastructure::utils};

use super::entities::User;

pub struct SignupUserRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

pub struct SigninUserRequest {
    pub email: String,
}

pub struct UpdateUserRequest {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl TryFrom<(User, user::usecase::requests::UpdateUserRequest)> for UpdateUserRequest {
    type Error = AppError;

    fn try_from(
        (user, request): (User, user::usecase::requests::UpdateUserRequest),
    ) -> Result<Self, Self::Error> {
        let password = match request.password {
            Some(naive_password) => {
                let password = utils::hasher::hash_password(&naive_password)?;
                password
            }
            None => user.password,
        };
        let email = request.email.map_or(user.email, |email| email);
        let username = request.username.map_or(user.username, |username| username);
        let bio = request.bio.map_or(user.bio, |bio| Some(bio));
        let image = request.image.map_or(user.image, |image| Some(image));

        Ok(UpdateUserRequest {
            id: user.id,
            email,
            username,
            password,
            bio,
            image,
        })
    }
}
