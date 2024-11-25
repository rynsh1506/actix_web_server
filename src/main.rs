use dotenvy::dotenv;
use web_server::{
    configs::config_load::{load_connection, load_env, load_tls_config},
    server,
    utils::logger,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logger::init();
    log::info!("Starting Actix application...");
    let config = load_env();
    let connection = load_connection(&config.db_url).await;
    if config.app_env == "production" {
        let tls_config = load_tls_config();
        server::start_server(config, connection, Some(tls_config)).await
    } else {
        server::start_server(config, connection, None).await
    }
}
