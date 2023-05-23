pub mod destination;
use crate::guard::employee::employee_guard;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("")
      .guard(actix_web::guard::fn_guard(employee_guard))
      .service(web::scope("/destinations").configure(destination::configure_routes)),
  );
}
