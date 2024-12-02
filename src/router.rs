use crate::{auth::auth_handler, server::AppState, users::users_handler};
use actix_web::web;

pub fn configure_v1(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/V1")
            .configure(auth_handler::configure)
            .configure(|cfg| users_handler::configure(cfg, app_state)),
    );
}

pub fn configure_v2(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    cfg.service(web::scope("/api/V2").configure(|cfg| users_handler::configure(cfg, app_state)));
}
