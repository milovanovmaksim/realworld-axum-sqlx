use serde::Serialize;
use utoipa::ToSchema;

use crate::app::domain;

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

impl From<domain::profile::usecase::responses::ProfileResponse> for ProfileResponse {
    fn from(value: domain::profile::usecase::responses::ProfileResponse) -> Self {
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
