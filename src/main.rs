use dotenvy::dotenv;
use web_server::{
    configs::config_load::{load_connection, load_env},
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

    server::start_server(config.clone(), connection, config.app_env == "producton").await
}
