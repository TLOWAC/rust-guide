use std::env;

use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Config {
            server: ServerConfig {
                host: env::var("HOST").unwrap(),
                port: env::var("PORT").unwrap().parse::<i32>().unwrap(),
            },
        })
    }
}
