#[derive(Debug)]
pub struct ProfileResponse {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}
