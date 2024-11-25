use sqlx::{Error, PgPool};

pub async fn establish_connection(db_url: &str) -> Result<PgPool, Error> {
    log::info!("Loading database connection...");
    let pool = PgPool::connect(db_url).await?;

    log::info!("Successfully connected to database");
    Ok(pool)
}
