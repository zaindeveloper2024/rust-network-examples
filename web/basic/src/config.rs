use serde::Deserialize;
use std::env;
use crate::error::ConfigError;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub environment: Environment,
    pub cors_origin: String,
}

/*
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_max_open: u32,
}
*/

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

const DEFAULT_PORT: u16 = 8080;

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        Ok(Config {
            app: AppConfig {
                host: Self::get_env_or("APP_HOST", "127.0.0.1")?,
                port: Self::get_env_or("APP_PORT", DEFAULT_PORT.to_string().as_str())?.parse()?,
                log_level: Self::get_env_or("RUST_LOG", "info")?,
                environment: Self::get_environment()?,
                cors_origin: Self::get_env_or("CORS_ORIGIN", "*")?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: Self::get_env_or("DATABASE_MAX_CONNECTIONS", "5")?.parse()?,
                timeout_seconds: Self::get_env_or("DATABASE_TIMEOUT_SECONDS", "5")?.parse()?,
            },
        })
    }

    fn get_environment() -> Result<Environment, ConfigError> {
        match std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()).as_str() {
            "production" => Ok(Environment::Production),
            "staging" => Ok(Environment::Staging),
            _ => Ok(Environment::Development),
        }
    }

    fn get_env_or(key: &str, default: &str) -> Result<String, ConfigError> {
        Ok(std::env::var(key).unwrap_or_else(|_| default.to_string()))
    }

    // pub fn get_cors_origins() -> Result<Vec<String>, ConfigError> {
    //     let origins = Self::get_env_or("CORS_ORIGINS", "*")?;
    //     Ok(origins.split(",").map(|s| s.to_string()).collect())
    // }

    // pub fn is_development(&self) -> bool {
    //     self.app.environment == Environment::Development
    // }

    // pub fn is_production(&self) -> bool {
    //     self.app.environment == Environment::Production
    // }
}
