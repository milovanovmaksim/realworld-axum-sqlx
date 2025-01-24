use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct SignupUserRequest {
    #[validate]
    pub user: SignupUser,
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct SignupUser {
    #[validate(
        required,
        length(min = 1, message = "username must be more than 1 character")
    )]
    pub username: Option<String>,

    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,

    #[validate(
        required,
        length(min = 8, message = "password must be more than 8 characters")
    )]
    pub password: Option<String>,
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct SigninUser {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,

    #[validate(
        required,
        length(min = 8, message = "password must be more than 8 characters")
    )]
    pub password: Option<String>,
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct SigninUserRequest {
    #[validate]
    pub user: SigninUser,
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate]
    pub user: UpdateUser,
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct UpdateUser {
    #[validate(length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,

    #[validate(length(min = 1, message = "username must be more than 1 character"))]
    pub username: Option<String>,
    #[validate(length(min = 8, message = "password must be more than 8 characters"))]
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}
