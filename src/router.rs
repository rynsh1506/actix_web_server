use crate::users::users_handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(users_handler::handler()));
}
