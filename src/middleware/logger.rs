use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use chrono::Utc;
use futures::future::{ok, LocalBoxFuture, Ready};
use log::{error, info, warn};
use std::task::{Context, Poll};
use std::time::Instant;

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerService { service })
    }
}

pub struct LoggerService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();
        let request_time = Utc::now();
        let wib_time =
            request_time.with_timezone(&chrono::offset::FixedOffset::east_opt(7 * 3600).unwrap());
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status();

            let formatted_time = wib_time.format("%Y-%m-%d %H:%M:%S").to_string();

            if status.is_client_error() {
                warn!(
                    "[{} {}] Status: {} - Duration: {:.3}ms - Request Time: {}",
                    method,
                    path,
                    status,
                    duration.as_secs_f64() * 1000.0,
                    formatted_time
                );
            } else if status.is_server_error() {
                error!(
                    "[{} {}] Status: {} - Duration: {:.3}ms - Request Time: {}",
                    method,
                    path,
                    status,
                    duration.as_secs_f64() * 1000.0,
                    formatted_time
                );
            } else {
                info!(
                    "[{} {}] Status: {} - Duration: {:.3}ms - Request Time: {}",
                    method,
                    path,
                    status,
                    duration.as_secs_f64() * 1000.0,
                    formatted_time
                );
            }

            Ok(res)
        })
    }
}
