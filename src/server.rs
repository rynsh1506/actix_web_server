use crate::{
    configs::config_env,
    middlewares::middleware_logger,
    router,
    utils::errors::{json_error_handler, query_error_handler},
};
use actix_cors::Cors;
use actix_web::{
    web::{self, JsonConfig, QueryConfig},
    App, HttpServer,
};
use rustls::ServerConfig;
use sqlx::PgPool;

pub async fn start_server(
    config: config_env::Config,
    connection: PgPool,
    tls: Option<ServerConfig>,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(QueryConfig::default().error_handler(query_error_handler))
            .app_data(JsonConfig::default().error_handler(json_error_handler))
            .wrap(middleware_logger::Logger)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(connection.clone()))
            .configure(router::configure)
    });

    if let Some(tls_config) = tls {
        server
            .bind_rustls_0_23((config.app_host, config.app_port), tls_config)?
            .run()
            .await
    } else {
        server.bind((config.app_host, config.app_port))?.run().await
    }
}
