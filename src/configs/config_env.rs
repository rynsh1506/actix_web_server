use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnv(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub app_env: String,
    pub app_host: String,
    pub app_port: u16,
    pub jwt_secret_key: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        log::info!("Loading configuration...");
        let db_url = env_var("DATABASE_URL", None)?;
        let app_env = {
            let app_env = env_var("APP_ENV", None)?;
            if app_env != "development" && app_env != "production" {
                return Err(ConfigError::InvalidValue(
                    "APP_ENV must be 'development' or 'production'".to_string(),
                ));
            }
            app_env
        };
        let app_host = env_var("APP_HOST", Some("localhost"))?;
        let app_port = env_var_u16("APP_PORT", 8080)?;
        let jwt_secret_key = env_var("JWT_SECRET_KEY", Some("dev-jwt-secret-key"))?;
        log::info!("Successfully loaded enviroment");

        Ok(Self {
            db_url,
            app_env,
            app_host,
            app_port,
            jwt_secret_key,
        })
    }
}

fn env_var(key: &str, default: Option<&str>) -> Result<String, ConfigError> {
    env::var(key).or_else(|_| {
        default
            .map(String::from)
            .ok_or(ConfigError::MissingEnv(key.into()))
    })
}

fn env_var_u16(key: &str, default: u16) -> Result<u16, ConfigError> {
    env_var(key, Some(&default.to_string())).and_then(|v| {
        v.parse()
            .map_err(|_| ConfigError::InvalidValue(format!("Invalid u16: {}", key)))
    })
}
