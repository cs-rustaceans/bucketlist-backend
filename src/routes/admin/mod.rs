pub mod destination;
pub mod user;
use crate::db::model::user::User;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("")
      .guard(actix_web::guard::fn_guard(|ctx| {
        let req_data = ctx.req_data();
        let user_optional = req_data.get::<User>();
        if user_optional.is_none() {
          return false;
        }
        if user_optional.unwrap().role != "admin" {
          return false;
        }
        return true;
      }))
      .service(web::scope("/users").configure(user::configure_routes))
      .service(web::scope("/destinations").configure(destination::configure_routes)),
  );
}
