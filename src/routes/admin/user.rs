use crate::applib::errors::AppError;
use crate::db::model::user::{NewUser, UpdateUser};
use crate::db::DbPool;
use crate::service::admin::user_service;

use crate::routes::common;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(user_service::admin_get_all_users(pool).await?),
  )
}

async fn get_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(user_service::admin_get_user_by_id(pool, *user_id).await?),
  )
}

async fn delete_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  user_service::admin_delete_user(pool, *user_id).await?;

  return Ok(HttpResponse::Ok().into());
}

async fn update_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
  user_json: web::Json<UpdateUser>,
) -> Result<HttpResponse, AppError> {
  user_service::admin_update_user(pool, *user_id, user_json).await?;

  return Ok(HttpResponse::Ok().into());
}

async fn create_user(
  pool: web::Data<DbPool>,
  user_json: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
  user_service::admin_create_user(pool, user_json).await?;

  return Ok(HttpResponse::Created().into());
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::resource("")
        .route(web::get().to(get_all_users))
        .route(web::post().to(create_user)),
    )
    .service(web::resource("/me").route(web::get().to(common::user::get_user)))
    .service(
      web::resource("/{id}")
        .route(web::get().to(get_user))
        .route(web::patch().to(update_user))
        .route(web::delete().to(delete_user)),
    );
}
