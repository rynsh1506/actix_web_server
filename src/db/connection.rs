use sqlx::Error;
use sqlx::PgPool;

pub async fn establish_connection(db_url: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(db_url).await?;

    log::info!("Successfully connected to database");
    Ok(pool)
}
