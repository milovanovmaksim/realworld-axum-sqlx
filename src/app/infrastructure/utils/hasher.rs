use bcrypt::{hash, BcryptResult, DEFAULT_COST};


///
/// Генерирует хеш пароля.
pub fn hash_password(naive_pw: &str) -> BcryptResult<String> {
    hash(naive_pw, DEFAULT_COST)
}

///
/// Проверяет, что пароль эквивалентен предоставленному хешу.
pub fn verify(naive_password: &str, hashed_password: &str) -> BcryptResult<bool> {
    bcrypt::verify(naive_password, hashed_password)
}
