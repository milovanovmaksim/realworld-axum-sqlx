use std::{fs::File, path::Path};

use serde::Deserialize;
use serde_yaml::Mapping;

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
    pub max_connections: u32,
}

impl DatabaseSettings {
    pub fn new(
        username: String,
        password: String,
        port: u16,
        host: String,
        database_name: String,
        require_ssl: bool,
        max_connections: u32,
    ) -> Self {
        Self {
            username,
            password,
            port,
            host,
            database_name,
            require_ssl,
            max_connections,
        }
    }

    pub fn from_yaml<T: AsRef<Path>>(path: T) -> Self {
        let file =
            File::open(path).expect("DatabaseSettings::form_yaml || error: failed to open file");
        let res: Mapping = serde_yaml::from_reader(file)
            .expect("DatabaseSettings::form_yaml || error: failed to parse data from file");
        let value = res
            .get("database")
            .expect("DatabaseSettings::form_yaml || error: failed to get database");
        let database_settings = DatabaseSettings::deserialize(value).expect(
            "DatabaseSettings::form_yaml || error: failed to deserialize database settings",
        );
        database_settings
    }
}
