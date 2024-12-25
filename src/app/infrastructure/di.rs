use std::{path::Path, sync::Arc};

use super::{
    jwt_token::{jwt_token::JwtAuthTokenImpl, settings::JwtTokenSettings},
    pgsql::{db::PostgreSQL, settings::DatabaseSettings},
    user::{repository::UsersRepositoryImpl, usecase::UserUseCaseImpl},
};

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: Arc<UsersRepositoryImpl>,
    pub user_usecase: Arc<UserUseCaseImpl>,

    /**
     *Utility services
     */
    pub jwt_auth_token: Arc<JwtAuthTokenImpl>,
}

impl DiContainer {
    pub async fn new<T: AsRef<Path>>(path: T) -> Self {
        // Utility services
        let jwt_auth_token = Arc::new(JwtAuthTokenImpl::new(JwtTokenSettings::from_yaml(
            path.as_ref(),
        )));
        let pg_sql =
            PostgreSQL::configure_database(DatabaseSettings::from_yaml(path.as_ref())).await;

        // User
        let user_repository = Arc::new(UsersRepositoryImpl::new(pg_sql.clone()));
        let user_usecase = Arc::new(UserUseCaseImpl::new(
            user_repository.clone(),
            jwt_auth_token.clone(),
        ));

        Self {
            user_repository,
            user_usecase,
            jwt_auth_token,
        }
    }
}
