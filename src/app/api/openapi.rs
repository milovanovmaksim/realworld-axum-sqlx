use utoipa::OpenApi;

use crate::app::api::user::{requests::SignupUserRequest, responses::SignupUserResponse};

#[derive(OpenApi)]
#[openapi(
    paths(crate::app::api::user::router::signup),
    components(
        schemas(SignupUserRequest, SignupUserResponse),
    ),
    tags(
        (name = "users", description = "Users endpoints")
    ),
)]
pub struct ApiDocunentation;
