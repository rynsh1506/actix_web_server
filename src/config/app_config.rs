use std::{env, error::Error};

#[derive(Debug)]
pub struct AppConfig {
    pub app_env: String,
    pub app_host: String,
    pub app_port: u16,
    pub jwt_secret_key: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let app_env = env::var("APP_ENV").map_err(|_| "Failed to get APP_ENV")?;

        if app_env != "development" && app_env != "production" {
            return Err("APP_ENV must be either 'development' or 'production'".into());
        }

        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let app_port = env::var("APP_PORT")?.parse().unwrap_or(8080);
        let jwt_secret_key =
            env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "dev-jwt-secret-key".to_string());

        Ok(Self {
            app_env,
            app_host,
            app_port,
            jwt_secret_key,
        })
    }
}
