#[cfg(test)]
mod test {
    use dotenvy::dotenv;
    use std::env;
    use web_server::configs::config_conn;

    #[tokio::test]
    async fn test_establish_connection() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/test_db".to_string());

        let result = config_conn::establish_connection(&db_url).await;

        assert!(
            result.is_ok(),
            "Failed to establish connection: {:?}",
            result.err()
        );
    }
}
