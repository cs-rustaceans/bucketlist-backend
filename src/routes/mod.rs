pub mod admin;
pub mod login;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/admin").configure(admin::configure_routes))
    .service(web::scope("/login").configure(login::configure_routes));
}
