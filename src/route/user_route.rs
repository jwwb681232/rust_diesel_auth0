use crate::handler::user_handler;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::get().to(user_handler::get_users)));
}
