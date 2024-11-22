pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub naive_password: String,
}

pub struct SigninRequest {
    pub email: String,
    pub naive_password: String,
}
