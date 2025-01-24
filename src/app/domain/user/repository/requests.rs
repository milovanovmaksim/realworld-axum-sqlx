use uuid::Uuid;

use crate::app::{
    domain::{error::AppError, user},
    infrastructure::utils,
};

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
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl TryFrom<user::usecase::requests::UpdateUserRequest> for UpdateUserRequest {
    type Error = AppError;

    fn try_from(value: user::usecase::requests::UpdateUserRequest) -> Result<Self, Self::Error> {
        let password = match value.password {
            Some(naive_password) => {
                let password = utils::hasher::hash_password(&naive_password)?;
                Some(password)
            }
            None => value.password,
        };
        Ok(UpdateUserRequest {
            id: value.id,
            email: value.email,
            username: value.username,
            password,
            bio: value.bio,
            image: value.image,
        })
    }
}
