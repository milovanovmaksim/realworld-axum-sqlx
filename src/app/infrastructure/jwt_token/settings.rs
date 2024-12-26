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

    pub fn from_yaml<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| {
            format!("DatabaseSettings::form_yaml || error: failed to open file {e}")
        })?;
        let res: Mapping = serde_yaml::from_reader(file).map_err(|e| {
            format!("JwtTokenSettings::form_yaml || error: failed to parse data from file {e}")
        })?;
        let value = res.get("jwt_token").ok_or(format!(
            "JwtTokenSettings::form_yaml || error: key 'jwt_token' does not exest "
        ))?;
        JwtTokenSettings::deserialize(value).map_err(|e| {
            format!("JwtTokenSettings::from_yaml || error: failed to deserialize JWT settings {e}")
        })
    }
}
