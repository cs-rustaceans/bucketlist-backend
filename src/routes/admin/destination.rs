use crate::applib::errors::AppError;
use crate::db::model::destination::Destination;
use crate::db::model::destination::NewDestination;
use crate::db::model::destination::UpdateDestination;
use crate::db::DbPool;
use crate::service::admin::destination_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_all_destinations(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
  let destinations: Vec<Destination> =
    destination_service::admin_get_all_destinations(pool).await?;
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(destinations),
  );
}

async fn get_all_unreviewed_destinations(
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
  let destinations: Vec<Destination> =
    destination_service::admin_get_all_unreviewed_destinations(pool).await?;
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(destinations),
  );
}

async fn get_destination_by_id(
  pool: web::Data<DbPool>,
  destination_id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  let destination: Destination =
    destination_service::admin_get_destination_by_id(pool, *destination_id).await?;
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(destination),
  );
}

async fn create_destination(
  pool: web::Data<DbPool>,
  new_destination_json: web::Json<NewDestination>,
) -> Result<HttpResponse, AppError> {
  destination_service::admin_create_destination(pool, new_destination_json).await?;
  return Ok(HttpResponse::Created().into());
}

async fn update_destination_by_id(
  pool: web::Data<DbPool>,
  destination_id: web::Path<u64>,
  update_destination_json: web::Json<UpdateDestination>,
) -> Result<HttpResponse, AppError> {
  destination_service::admin_update_destination_by_id(
    pool,
    *destination_id,
    update_destination_json,
  )
  .await?;
  return Ok(HttpResponse::Ok().into());
}

async fn delete_destination_by_id(
  pool: web::Data<DbPool>,
  destination_id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  destination_service::admin_delete_destination_by_id(pool, *destination_id).await?;
  return Ok(HttpResponse::Ok().into());
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::resource("")
        .route(web::get().to(get_all_destinations))
        .route(web::post().to(create_destination)),
    )
    .service(web::resource("/unreviewed").route(web::get().to(get_all_unreviewed_destinations)))
    .service(
      web::resource("/{id}")
        .route(web::get().to(get_destination_by_id))
        .route(web::patch().to(update_destination_by_id))
        .route(web::delete().to(delete_destination_by_id)),
    );
}
