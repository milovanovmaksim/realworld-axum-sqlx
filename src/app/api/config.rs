use std::{fs::File, path::Path};

use serde::Deserialize;
use serde_yaml::Mapping;

#[derive(Deserialize)]
pub struct ApiConfig {
    host: String,
    port: u32,
    frontend_origin: String,
}

impl ApiConfig {
    pub fn from_yaml<T: AsRef<Path>>(path: T) -> Self {
        let file =
            File::open(path).expect("DatabaseSettings::form_yaml || error: failed to open file");
        let res: Mapping = serde_yaml::from_reader(file)
            .expect("DatabaseSettings::form_yaml || error: failed to parse data from file");
        let value = res
            .get("api")
            .expect("ApiConfig::form_yaml || error: key 'api' does not exist");
        let config = ApiConfig::deserialize(value).expect(
            "DatabaseSettings::form_yaml || error: failed to deserialize database settings",
        );
        config
    }
}
