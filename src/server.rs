use crate::{
    configs::config_env,
    middlewares::middleware_logger,
    router::{configure_v1, configure_v2},
    utils::errors::{
        json_error_handler, path_error_handler, qs_query_error_handler, query_error_handler,
    },
};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use chrono::Duration;
use rustls::ServerConfig;
use serde_qs::actix::QsQueryConfig;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub secret_key: Arc<String>,
    pub jwt_expiration_time: Arc<Duration>,
    pub jwt_refresh_expiration_time: Arc<Duration>,
}

pub async fn start_server(
    config: config_env::Config,
    connection: PgPool,
    tls_config: ServerConfig,
    is_secure: bool,
) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        secret_key: Arc::new(config.jwt_secret_key),
        jwt_expiration_time: Arc::new(config.jwt_expiration_time),
        jwt_refresh_expiration_time: Arc::new(config.jwt_refresh_expiration_time),
    });

    let server = HttpServer::new(move || {
        let cors_config = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::QueryConfig::default().error_handler(query_error_handler))
            .app_data(QsQueryConfig::default().error_handler(qs_query_error_handler))
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .app_data(web::PathConfig::default().error_handler(path_error_handler))
            .wrap(middleware_logger::LoggerMiddleware)
            .wrap(cors_config)
            .app_data(web::Data::new(connection.clone()))
            .app_data(app_state.clone())
            .configure(|cfg| {
                if is_secure {
                    configure_v2(cfg, app_state.clone());
                } else {
                    configure_v1(cfg, app_state.clone());
                }
            })
    });

    if is_secure {
        server
            .bind_rustls_0_23((config.app_host, config.app_port), tls_config)?
            .run()
            .await
    } else {
        server.bind((config.app_host, config.app_port))?.run().await
    }
}
