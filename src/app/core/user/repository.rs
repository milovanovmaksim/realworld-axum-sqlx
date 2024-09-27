use super::entity::User;

pub trait UserRepository {
    fn signup(&self, email: &str, username: &str, naive_password: &str) -> Result<User, AppError>
}
