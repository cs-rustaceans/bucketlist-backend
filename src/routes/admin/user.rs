use crate::applib::errors::AppError;
use crate::db::model::user::{NewUser, UpdateUser};
use crate::db::DbPool;
use crate::dto::get_user_dto::GetUserDTO;
use crate::service::admin::user_service;

use crate::routes::common;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_all_users(
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<Vec<GetUserDTO>, AppError> = user_service::admin_get_all_users(pool).await;
  if let Err(error) = result {
    return Err(error);
  } else {
    let users: Vec<GetUserDTO> = result.unwrap();
    return Ok(
      HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users),
    );
  }
}

async fn get_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<GetUserDTO, AppError> =
    user_service::admin_get_user_by_id(pool, *user_id).await;
  if let Err(error) = result {
    return Err(error);
  } else {
    let user: GetUserDTO = result.unwrap();
    return Ok(
      HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(user),
    );
  }
}

async fn delete_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<(), AppError> = user_service::admin_delete_user(pool, *user_id).await;

  if let Err(error) = result {
    return Err(error);
  } else {
    return Ok(HttpResponse::Ok().into());
  }
}

async fn update_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
  user_json: web::Json<UpdateUser>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<(), AppError> =
    user_service::admin_update_user(pool, *user_id, user_json).await;

  if let Err(error) = result {
    return Err(error);
  } else {
    return Ok(HttpResponse::Ok().into());
  }
}

async fn create_user(
  pool: web::Data<DbPool>,
  user_json: web::Json<NewUser>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<(), AppError> = user_service::admin_create_user(pool, user_json).await;

  if let Err(error) = result {
    return Err(error);
  } else {
    return Ok(HttpResponse::Created().into());
  }
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
