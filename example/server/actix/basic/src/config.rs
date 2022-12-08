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
                host: get_env_var("HOST"),
                port: get_env_var("PORT"),
            },
        })
    }
}

fn get_env_var<T>(env_var_name: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let var = env::var(env_var_name).unwrap();
    return var.parse::<T>().unwrap();
}
