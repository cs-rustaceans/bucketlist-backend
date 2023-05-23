pub mod admin;
pub mod common;
pub mod employee;
pub mod login;
use crate::middleware::auth_middleware::authentification_middleware;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/login").configure(login::configure_routes))
    .service(
      web::scope("")
        .wrap(HttpAuthentication::bearer(authentification_middleware))
        .service(web::scope("/admin").configure(admin::configure_routes))
        .service(web::scope("/employee").configure(employee::configure_routes)),
    );
}
