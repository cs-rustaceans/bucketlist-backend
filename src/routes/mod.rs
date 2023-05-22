pub mod admin;
pub mod login;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::middleware::auth_middleware::authentification_middleware;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/login").configure(login::configure_routes))
    .service(
      web::scope("/admin")
        .wrap(HttpAuthentication::bearer(authentification_middleware))
        .configure(admin::configure_routes),
    );
}
