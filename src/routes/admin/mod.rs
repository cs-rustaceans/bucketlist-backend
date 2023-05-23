pub mod destination;
pub mod user;
use actix_web::web;
use crate::guard::admin::admin_guard;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("")
      .guard(actix_web::guard::fn_guard(admin_guard))
      .service(web::scope("/users").configure(user::configure_routes))
      .service(web::scope("/destinations").configure(destination::configure_routes)),
  );
}
