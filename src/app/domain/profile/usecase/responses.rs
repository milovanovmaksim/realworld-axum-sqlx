use crate::app::domain::user::repository::entities::UserEntity;

///
/// Ответ, возвращаемый слоем бизнес-логики профиля.
/// Содержит информацию о профиле.
#[derive(Debug)]
pub struct ProfileResponse {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<(bool, UserEntity)> for ProfileResponse {
    fn from((following, profile): (bool, UserEntity)) -> Self {
        ProfileResponse {
            username: profile.username,
            bio: profile.bio,
            image: profile.image,
            following,
        }
    }
}
