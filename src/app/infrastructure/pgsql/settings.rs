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

    pub fn from_yaml<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let file = File::open(path.as_ref()).map_err(|e| {
            format!("DatabaseSettings::form_yaml || error: failed to open file {e}")
        })?;
        let res: Mapping = serde_yaml::from_reader(file).map_err(|_| {
            format!(
                "DatabaseSettings::form_yaml || error: failed to parse data from file {}.",
                path.as_ref().to_string_lossy()
            )
        })?;
        let value = res
            .get("database")
            .ok_or("DatabaseSettings::from_yaml || error: key 'database' was not found.")?;
        let database_settings = DatabaseSettings::deserialize(value).map_err(|e| {
            format!(
                "DatabaseSettings::form_yaml || error: failed to deserialize database settings {e}"
            )
        })?;
        Ok(database_settings)
    }
}
