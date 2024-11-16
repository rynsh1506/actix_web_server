use dotenvy::dotenv;
use web_server::config::{load_config, load_db_connection, load_tls_config};
use web_server::server::{start_http_server, start_https_server};
use web_server::utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    utils::init_logger();
    log::info!("Starting Actix application...");

    let config = load_config().await?;
    let connection = load_db_connection(&config).await?;

    if config.app_config.app_env.eq("production") {
        let server_config = load_tls_config().await?;
        start_https_server(config, connection, server_config).await
    } else {
        start_http_server(config, connection).await
    }
}
