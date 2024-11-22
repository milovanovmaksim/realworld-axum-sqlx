pub struct SignupRequest {
    username: String,
    email: String,
    naive_password: String,
}

pub struct SigninRequest {
    email: String,
    hashed_password: String,
}
