pub struct SignupUserRepositoryRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

pub struct SigninUserRepositoryRequest {
    pub email: String,
}
