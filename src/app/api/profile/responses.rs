use serde::Serialize;

use crate::app::domain;

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    profile: ProfileInner,
}

#[derive(Debug, Serialize)]
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
