use crate::config::certs_config;
use crate::config::Config;
use crate::db;
use std::io::Result;

pub async fn load_config() -> Result<Config> {
    match Config::init() {
        Ok(config) => Ok(config),
        Err(e) => {
            log::error!("Error loading config: {:?}", e);
            std::process::exit(1);
        }
    }
}

pub async fn load_db_connection(config: &Config) -> Result<sqlx::Pool<sqlx::Postgres>> {
    match db::establish_connection(&config.db_config.db_url).await {
        Ok(connection) => Ok(connection),
        Err(e) => {
            log::error!("Error establishing database connection: {:?}", e);
            std::process::exit(1);
        }
    }
}

pub async fn load_tls_config() -> Result<rustls::ServerConfig> {
    match certs_config() {
        Ok(config) => Ok(config),
        Err(e) => {
            log::error!("Error loading TLS certificates: {:?}", e);
            std::process::exit(1);
        }
    }
}
