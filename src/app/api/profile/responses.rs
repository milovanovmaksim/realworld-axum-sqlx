use serde::Serialize;
use utoipa::ToSchema;

use crate::app::domain::profile::usecase::responses::ProfileUsecase;

///
/// Ответ api сервера.
/// Содержит информацию о профиле.
#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileResponse {
    profile: ProfileInner,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileInner {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<ProfileUsecase> for ProfileResponse {
    fn from(value: ProfileUsecase) -> Self {
        ProfileResponse {
            profile: ProfileInner {
                username: value.username,
                bio: value.bio,
                image: value.image,
                following: value.following,
            },
        }
    }
}
