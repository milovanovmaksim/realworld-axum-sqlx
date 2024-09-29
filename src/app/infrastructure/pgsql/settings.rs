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
}
