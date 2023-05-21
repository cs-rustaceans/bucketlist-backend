use crate::applib::errors::AppError;
use crate::db::model::destination::Destination;
use crate::db::DbPool;
use crate::service::admin::destination_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_all_destinations(
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<Vec<Destination>, AppError> =
    destination_service::admin_get_all_destinations(pool).await;
  if let Err(error) = result {
    return Err(error);
  } else {
    let users: Vec<Destination> = result.unwrap();
    return Ok(
      HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users),
    );
  }
}

async fn get_all_unreviewed_destinations(
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<Vec<Destination>, AppError> =
    destination_service::admin_get_all_unreviewed_destinations(pool).await;
  if let Err(error) = result {
    return Err(error);
  } else {
    let users: Vec<Destination> = result.unwrap();
    return Ok(
      HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users),
    );
  }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("").route(web::get().to(get_all_destinations)))
    .service(web::resource("/unreviewed").route(web::get().to(get_all_unreviewed_destinations)));
}
