pub struct SignupResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: OffsetDateTime,
}

pub struct SigninResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
