pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

pub struct SigninRequest {
    pub email: String,
}
