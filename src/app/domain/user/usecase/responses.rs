pub struct SignupResponse {
    pub user: AuthUser,
}

pub struct AuthUser {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}

pub struct SigninResponse {
    pub user: AuthUser,
}
