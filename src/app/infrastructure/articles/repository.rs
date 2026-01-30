use std::any::Any;

use async_trait::async_trait;
use sqlx::{query_file_as, PgConnection};
use tracing::error;

use crate::app::{
    domain::articles::repository::{
        entities::Article, requests::CreateArticleRepoRequest, ArticlesRepository,
    },
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};

pub struct ArticlesRepositoryImpl {
    pg_sql: PostgreSQL,
}

impl ArticlesRepositoryImpl {
    pub fn new(pg_sql: PostgreSQL) -> Self {
        Self { pg_sql }
    }
}

#[async_trait]
impl ArticlesRepository for ArticlesRepositoryImpl {
    async fn create_article(
        &self,
        ctx: Option<&mut (dyn Any + Send + Sync)>,
        article: CreateArticleRepoRequest,
    ) -> Result<Article, AppError> {
        let query = query_file_as!(
            Article,
            "./src/app/infrastructure/queries/articles/insert.sql",
            article.title,
            article.body,
            article.description,
            article.slug,
            article.user_id
        );

        if let Some(conn) = ctx {
            if let Some(connection) = conn.downcast_mut::<PgConnection>() {
                Ok(query.fetch_one(connection).await?)
            } else {
                error!("Error while downcasting to PgConnection type.");
                Err(AppError::InternalServerError)
            }
        } else {
            Ok(query.fetch_one(&self.pg_sql.pool()).await?)
        }
    }
}
