use std::env;

use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
        pub server: ServerSettings,
        pub database: DatabaseSettings,
        pub phase: PhaseSettings,
}
#[derive(Deserialize, Debug)]
pub struct ServerSettings {
        pub host: String,
        pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
        pub username: String,
        pub password: String,
        pub database_name: String,
        pub host: String,
        pub url: String,
        pub port: i32,
}

#[derive(Deserialize, Debug)]
pub struct PhaseSettings {
        pub rust_env: String,
        pub is_production: bool,
        pub is_development: bool,
        pub is_local: bool,
}

impl Settings {
        pub fn from_env() -> Result<Self, ConfigError> {
                Ok(Settings {
                        server: ServerSettings {
                                host: get_env_var("HOST"),
                                port: get_env_var("PORT"),
                        },
                        database: DatabaseSettings {
                                username: get_env_var("DATABASE_USERNAME"),
                                password: get_env_var("DATABASE_PASSWORD"),
                                database_name: get_env_var("DATABASE_NAME"),
                                host: get_env_var("DATABASE_HOST"),
                                url: get_env_var("DATABASE_URL"),
                                port: get_env_var("DATABASE_PORT"),
                        },
                        phase: PhaseSettings {
                                rust_env: get_env_var("RUST_ENV"),
                                is_production: true,
                                is_development: true,
                                is_local: true,
                        },
                })
        }
}

impl DatabaseSettings {
        pub fn get_connection_url(&self) -> String {
                return format!(
                        "postgres://{}:{}@{}:{}/{}",
                        self.username, self.password, self.host, self.port, self.database_name
                );
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
