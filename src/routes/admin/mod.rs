pub mod user;
pub mod destination;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
      .service(web::scope("/users").configure(user::configure_routes))
      .service(web::scope("/destinations").configure(destination::configure_routes));
}
