use std::{fs::File, path::Path};

use serde::Deserialize;
use serde_yaml::Mapping;

#[derive(Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub frontend_origin: String,
}

impl ApiConfig {
    pub fn from_yaml<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| {
            format!("DatabaseSettings::form_yaml || error: failed to open file {e}")
        })?;
        let res: Mapping = serde_yaml::from_reader(file).map_err(|e| {
            format!("DatabaseSettings::form_yaml || error: failed to parse data from file {e}")
        })?;
        let value = res
            .get("api")
            .ok_or("ApiConfig::form_yaml || error: key 'api' does not exist")?;
        let config = ApiConfig::deserialize(value).map_err(|e| {
            format!("ApiConfig::form_yaml || error: failed to deserialize api settings {e}",)
        })?;
        Ok(config)
    }
}
