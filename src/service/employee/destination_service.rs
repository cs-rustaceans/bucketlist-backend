use crate::applib::errors::AppError;
use crate::db::model::destination::Destination;
use crate::db::model::user::User;
use crate::db::predicates::destination::available_for_user;
use crate::db::schema::destinations::dsl::*;
use crate::db::DbPool;
use actix_web::web;
use diesel::prelude::*;

pub async fn employee_get_all_available_destinations(
  db_pool: web::Data<DbPool>,
  user: User,
) -> Result<Vec<Destination>, AppError> {
  Ok(
    web::block(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      destinations
        .filter(available_for_user(user.id))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??,
  )
}

pub async fn employee_get_single_available_destination_by_id(
  db_pool: web::Data<DbPool>,
  user: User,
  destination_id: u64,
) -> Result<Destination, AppError> {
  Ok(
    web::block(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      destinations
        .filter(available_for_user(user.id).and(id.eq(destination_id)))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??
    .pop()
    .ok_or(AppError::not_found(Some(String::from(
      "available destination",
    ))))?,
  )
}
