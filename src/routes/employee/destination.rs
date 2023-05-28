use crate::applib::errors::AppError;
use crate::db::model::destination::Destination;
use crate::db::model::user::User;
use crate::db::DbPool;
use crate::service::employee::destination_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_all_available_destinations(
  pool: web::Data<DbPool>,
  user: User,
) -> Result<HttpResponse, AppError> {
  let destinations: Vec<Destination> =
    destination_service::employee_get_all_available_destinations(pool, user).await?;
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(destinations),
  );
}

async fn get_single_available_destination(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  let destination =
    destination_service::employee_get_single_available_destination_by_id(pool, user, *id).await?;
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(destination),
  );
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("").route(web::get().to(get_all_available_destinations)))
    .service(web::resource("/{id}").route(web::get().to(get_single_available_destination)));
}
