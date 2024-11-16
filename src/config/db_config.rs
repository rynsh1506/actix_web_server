use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct DbConfig {
    pub db_url: String,
}

impl DbConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let db_url = env::var("DATABASE_URL").map_err(|_| "Failed to get DATABASE_URL")?;

        Ok(Self { db_url })
    }
}
