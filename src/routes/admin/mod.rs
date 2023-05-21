pub mod user;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/users").configure(user::configure_routes));
}
