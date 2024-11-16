use crate::config::{AppConfig, DbConfig};
use log;

#[derive(Debug)]
pub struct Config {
    pub app_config: AppConfig,
    pub db_config: DbConfig,
}

struct ConfigBuilder {
    app_config: Option<AppConfig>,
    db_config: Option<DbConfig>,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            app_config: None,
            db_config: None,
        }
    }

    fn with_app_config(mut self, app_config: AppConfig) -> Self {
        self.app_config = Some(app_config);
        self
    }

    fn with_db_config(mut self, db_config: DbConfig) -> Self {
        self.db_config = Some(db_config);
        self
    }

    fn build(self) -> Result<Config, String> {
        let app_config = self.app_config.ok_or("Missing app config".to_string())?;
        let db_config = self.db_config.ok_or("Missing DB config".to_string())?;

        log::info!("Successfully loaded environment variables.");
        Ok(Config {
            app_config,
            db_config,
        })
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}

impl Config {
    pub fn init() -> Result<Config, String> {
        let app_config =
            AppConfig::new().map_err(|e| format!("Error loading app config: {}", e))?;
        let db_config = DbConfig::new().map_err(|e| format!("Error loading DB config: {}", e))?;

        ConfigBuilder::new()
            .with_app_config(app_config)
            .with_db_config(db_config)
            .build()
    }
}
