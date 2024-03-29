pub mod bucketlist_item;
pub mod destination;
pub mod user;
use crate::guard::admin::admin_guard;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("")
      .guard(actix_web::guard::fn_guard(admin_guard))
      .service(web::scope("/users").configure(user::configure_routes))
      .service(web::scope("/destinations").configure(destination::configure_routes))
      .service(web::scope("/bucketlist-items").configure(bucketlist_item::configure_routes)),
  );
}
