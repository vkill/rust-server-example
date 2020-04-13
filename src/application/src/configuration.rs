use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct HTTPServer {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub http_server: HTTPServer,
}

impl Configuration {
    pub fn new(base_path: PathBuf) -> Result<Self, ConfigError> {
        let mut config = Config::new();

        let path = base_path.join("configuration/base");
        config.merge(File::from(path))?;

        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let path = base_path.join(&format!("configuration/{}", environment));
        config.merge(File::from(path).required(false))?;

        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // Eg.. `APP_HTTP_SERVER__PORT=5001 would set `Configuration.http_server.port`
        config.merge(
            Environment::with_prefix("app")
                .separator("__")
                .ignore_empty(true),
        )?;

        config.try_into()
    }
}
