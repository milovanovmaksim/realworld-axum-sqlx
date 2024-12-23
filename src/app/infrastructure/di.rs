use std::{path::Path, sync::Arc};

use super::{
    jwt_token::{jwt_token::JwtAuthTokenImpl, settings::JwtTokenSettings},
    pgsql::{db::PostgreSQL, settings::DatabaseSettings},
    user::{repository::UsersRepositoryImpl, usecase::UserUseCaseImpl},
};

pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UsersRepositoryImpl,
    pub user_usecase: UserUseCaseImpl,

    pub jwt_auth_token: JwtAuthTokenImpl,
}

impl DiContainer {
    pub async fn new<T: AsRef<Path>>(path: T) -> Self {
        let jwt_auth_token = JwtAuthTokenImpl::new(JwtTokenSettings::from_yaml(path.as_ref()));
        let pg_sql =
            PostgreSQL::configure_database(DatabaseSettings::from_yaml(path.as_ref())).await;

        // User
        let user_repository = UsersRepositoryImpl::new(pg_sql.clone());
        let user_usecase = UserUseCaseImpl::new(
            Arc::new(user_repository.clone()),
            Arc::new(jwt_auth_token.clone()),
        );

        Self {
            user_repository,
            user_usecase,
            jwt_auth_token,
        }
    }
}
