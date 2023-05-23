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

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("").route(web::get().to(get_own_bucketlist)));
}
