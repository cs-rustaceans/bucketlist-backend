use crate::applib::errors::AppError;
use crate::db::model::user::User;
use crate::db::DbPool;
use crate::service::employee::bucketlist_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_own_bucketlist(pool: web::Data<DbPool>, user: User) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(bucketlist_service::employee_get_own_bucketlist(pool, user).await?),
  )
}

async fn get_bucketlist_item_by_id(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(bucketlist_service::employee_get_bucketlist_item_by_id(pool, user, *id).await?),
  )
}
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("").route(web::get().to(get_own_bucketlist)))
    .service(
      web::scope("/{id}").service(web::resource("").route(web::get().to(get_bucketlist_item_by_id))),
    );
}
