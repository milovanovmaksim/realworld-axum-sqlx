use std::{fs::File, path::Path};

use serde::Deserialize;
use serde_yaml::Mapping;

#[derive(Deserialize, Debug, Clone)]
pub struct JwtTokenSettings {
    pub secret_key: String,
}

impl JwtTokenSettings {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn from_yaml<T: AsRef<Path>>(path: T) -> Self {
        let file =
            File::open(path).expect("DatabaseSettings::form_yaml || error: failed to open file");
        let res: Mapping = serde_yaml::from_reader(file)
            .expect("JwtTokenSettings::form_yaml || error: failed to parse data from file");
        let value = res
            .get("jwt_token")
            .expect("JwtTokenSettings::form_yaml || error: failed to deserialize JWT settings");
        JwtTokenSettings::deserialize(value)
            .expect("JwtTokenSettings::from_yaml || error: failed to deserialize JWT settings.")
    }
}
