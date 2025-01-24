use uuid::Uuid;

use crate::app::domain::user::usecase::requests as user_usacase_requests;

pub struct SignupUserRepositoryRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

pub struct SigninUserRepositoryRequest {
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

impl From<user_usacase_requests::UpdateUserRequest> for UpdateUserRequest {
    fn from(value: user_usacase_requests::UpdateUserRequest) -> Self {
        todo!("Проверить, если пароль передали, то создать хэш и предать в запрос.");
        UpdateUserRequest {
            id: value.id,
            email: value.email,
            username: value.username,
            password: value.password,
            bio: value.bio,
            image: value.image,
        }
    }
}
