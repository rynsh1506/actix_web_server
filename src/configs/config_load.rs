use crate::configs::{
    config_conn::establish_connection, config_env::Config, config_tls::certs_config,
};
use rustls::ServerConfig;
use sqlx::PgPool;

pub fn load_env() -> Config {
    Config::new().unwrap_or_else(|e| {
        log::error!("Failed to load configuration: {}", e);
        std::process::exit(1);
    })
}

pub fn load_tls_config() -> ServerConfig {
    certs_config().unwrap_or_else(|e| {
        log::error!("Failed to load TLS configuration: {}", e);
        std::process::exit(1);
    })
}

pub async fn load_connection(db_url: &str) -> PgPool {
    establish_connection(db_url).await.unwrap_or_else(|e| {
        log::error!("Failed to establish database connection: {}", e);
        std::process::exit(1);
    })
}
