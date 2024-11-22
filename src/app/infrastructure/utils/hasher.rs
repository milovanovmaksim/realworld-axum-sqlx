use bcrypt::{hash, BcryptResult, DEFAULT_COST};

pub fn hash_password(naive_pw: &str) -> BcryptResult<String> {
    hash(naive_pw, DEFAULT_COST)
}

pub fn verify(naive_password: &str, hashed_password: &str) -> BcryptResult<bool> {
    bcrypt::verify(naive_password, hashed_password)
}
