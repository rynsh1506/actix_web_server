#[cfg(test)]
mod tests {
    use actix_web::web;
    use actix_web::{test, App};
    use web_server::routes::health_check;

    #[actix_web::test]
    async fn test_health_check() {
        // Setup the test server with the health check service
        let app =
            test::init_service(App::new().route("/health", web::get().to(health_check))).await;

        // Make a GET request to the health check endpoint
        let req = test::TestRequest::get().uri("/health").to_request();

        // Call the request and read the response
        let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp["status"], "ok");
    }
}
