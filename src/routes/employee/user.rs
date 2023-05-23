use crate::routes::common;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/me").route(web::get().to(common::user::get_user)));
}
