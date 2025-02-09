use async_trait::async_trait;
use sqlx::{postgres::PgRow, QueryBuilder, Row};

use crate::app::{
    domain::tags::repository::{responses::TagEntity, TagsRepository},
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};

///
/// Реализует интерфейс TagsRepository для работы с таблицей 'tags' базы данных PostgreSQL.
pub struct TagsRepositoryImpl {
    pg_sql: PostgreSQL,
}

impl TagsRepositoryImpl {
    pub fn new(pg_sql: PostgreSQL) -> Self {
        Self { pg_sql }
    }
}

#[async_trait]
impl TagsRepository for TagsRepositoryImpl {
    ///
    /// Возвращает список тэгов по имени.
    async fn get_tags(&self, tags: Vec<String>) -> Result<Vec<TagEntity>, AppError> {
        let mut query_builder = QueryBuilder::new("select id, tag, created_at from tags ");

        if !tags.is_empty() {
            query_builder
                .push("where tag = any(")
                .push_bind(tags)
                .push(")");
        }

        query_builder
            .push("order by tag")
            .build()
            .map(|row: PgRow| TagEntity {
                id: row.get(0),
                tag: row.get(1),
                created_at: row.get(2),
            })
            .fetch_all(&self.pg_sql.pool())
            .await
            .map_err(|err| AppError::from(err))
    }

    ///
    /// Добавляет новые тэги в БД.
    async fn create_tags(&self, tags: Vec<String>) -> Result<Vec<TagEntity>, AppError> {
        let mut query_builder = QueryBuilder::new("insert into tags (tag) ");

        query_builder.push_values(tags, |mut builder, tag| {
            builder.push_bind(tag);
        });

        query_builder
            .push("returning *")
            .build()
            .map(|row: PgRow| TagEntity {
                id: row.get(0),
                tag: row.get(1),
                created_at: row.get(2),
            })
            .fetch_all(&self.pg_sql.pool())
            .await
            .map_err(|err| AppError::from(err))
    }
}
