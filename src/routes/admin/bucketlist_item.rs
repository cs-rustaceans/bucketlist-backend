use crate::applib::errors::AppError;
use crate::db::DbPool;
use crate::service::admin::bucketlist_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_favorite_bucketlist_items(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(bucketlist_service::admin_get_favorite_bucketlist_items(pool).await?),
  )
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("").route(web::get().to(get_favorite_bucketlist_items)));
}
