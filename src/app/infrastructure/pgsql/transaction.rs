use std::ops::Deref;

use super::db::PostgreSQL;

pub struct Transaction {
    pg_sql: PostgreSQL,
}

impl Transaction {
    pub fn new(pg_sql: PostgreSQL) -> Self {
        Self { pg_sql }
    }
}

impl Deref for Transaction {
    type Target = PostgreSQL;

    fn deref(&self) -> &Self::Target {
        &self.pg_sql
    }
}
