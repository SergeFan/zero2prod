//! src/configuration.rs
use std::env;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let database_host = env::var("DB_HOST").unwrap_or(format!("{}:{}", self.host, self.port));
        let database_name = env::var("DB_NAME").unwrap_or(self.database_name.clone());
        let database_user =
            env::var("DB_USER").unwrap_or(format!("{}:{}", self.username, self.password));

        format!(
            "postgres://{}@{}/{}",
            database_user, database_host, database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        let database_host = env::var("DB_HOST").unwrap_or(format!("{}:{}", self.host, self.port));
        let database_user =
            env::var("DB_USER").unwrap_or(format!("{}:{}", self.username, self.password));

        format!("postgres://{}@{}", database_user, database_host)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    // Try to convert the configuration values it read into Setting type
    settings.try_deserialize::<Settings>()
}
