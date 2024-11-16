use crate::{config::Config, middleware::Logger};
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

pub async fn start_http_server(config: Config, connection: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger)
            .app_data(web::Data::new(connection.clone()))
    })
    .bind((config.app_config.app_host, config.app_config.app_port))
    .unwrap_or_else(|_| {
        log::error!(
            "Port {} is already in use. Server will not start.",
            config.app_config.app_port
        );
        std::process::exit(1);
    })
    .run()
    .await
}

pub async fn start_https_server(
    config: Config,
    connection: PgPool,
    server_config: rustls::ServerConfig,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger)
            .app_data(web::Data::new(connection.clone()))
    })
    .bind_rustls_0_23(
        (config.app_config.app_host, config.app_config.app_port),
        server_config,
    )
    .unwrap_or_else(|_| {
        log::error!(
            "Port {} is already in use. Server will not start.",
            config.app_config.app_port
        );
        std::process::exit(1);
    })
    .run()
    .await
}
