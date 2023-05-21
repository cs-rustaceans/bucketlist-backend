use crate::applib::errors::AppError;
use crate::db::model::destination::Destination;
use crate::db::schema::destinations::dsl::*;
use crate::db::DbPool;
use actix_web::web;
use diesel::prelude::*;

pub async fn admin_get_all_destinations(
  db_pool: web::Data<DbPool>,
) -> Result<Vec<Destination>, AppError> {
  let result: Result<Result<Vec<Destination>, AppError>, _> = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }
    destinations
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(inner_result) => inner_result,
    Err(_) => Err(AppError::internal_server_error()),
  }
}

pub async fn admin_get_all_unreviewed_destinations(
  db_pool: web::Data<DbPool>,
) -> Result<Vec<Destination>, AppError> {
  let result: Result<Result<Vec<Destination>, AppError>, _> = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }
    destinations
      .filter(isReviewed.eq(false))
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(inner_result) => inner_result,
    Err(_) => Err(AppError::internal_server_error()),
  }
}
