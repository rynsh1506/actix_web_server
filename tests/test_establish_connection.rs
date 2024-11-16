#[cfg(test)]
mod tests {
    use std::env;
    use web_server::db::establish_connection;

    #[tokio::test]
    async fn test_establish_connection() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/test_db".to_string());

        let result = establish_connection(&db_url).await;

        assert!(
            result.is_ok(),
            "Failed to establish connection: {:?}",
            result.err()
        );
        println!("Connection test passed!");
    }
}
