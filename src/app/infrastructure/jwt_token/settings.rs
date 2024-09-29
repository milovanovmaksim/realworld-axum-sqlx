pub struct JwtTokenSettings {
    pub secret_key: String,
}

impl JwtTokenSettings {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}
