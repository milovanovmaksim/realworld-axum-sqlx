use crate::app::domain::user::repository::entities::User;

#[derive(Debug)]
pub struct ProfileResponse {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<(bool, User)> for ProfileResponse {
    fn from((following, profile): (bool, User)) -> Self {
        ProfileResponse {
            username: profile.username,
            bio: profile.bio,
            image: profile.image,
            following,
        }
    }
}
