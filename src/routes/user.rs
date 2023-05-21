use crate::applib::errors::AppError;
use crate::db::model::user::{NewUser, UpdateUser, User};
use crate::db::DbPool;
use crate::service::user_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn get_all_users(
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<Vec<User>, AppError> = user_service::get_all_users(pool).await;
  if let Err(error) = result {
    return Err(error);
  } else {
    let users: Vec<User> = result.unwrap();
    return Ok(
      HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users),
    );
  }
}

pub async fn get_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<User, AppError> = user_service::get_user_by_id(pool, *user_id).await;
  if let Err(error) = result {
    return Err(error);
  } else {
    let user: User = result.unwrap();
    return Ok(
      HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(user),
    );
  }
}

pub async fn delete_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<(), AppError> = user_service::delete_user(pool, *user_id).await;

  if let Err(error) = result {
    return Err(error);
  } else {
    return Ok(HttpResponse::Ok().into());
  }
}

pub async fn update_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<u64>,
  user_json: web::Json<UpdateUser>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<(), AppError> = user_service::update_user(pool, *user_id, user_json).await;

  if let Err(error) = result {
    return Err(error);
  } else {
    return Ok(HttpResponse::Ok().into());
  }
}

pub async fn create_user(
  pool: web::Data<DbPool>,
  user_json: web::Json<NewUser>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result: Result<(), AppError> = user_service::create_user(pool, user_json).await;

  if let Err(error) = result {
    return Err(error);
  } else {
    return Ok(HttpResponse::Created().into());
  }
}
