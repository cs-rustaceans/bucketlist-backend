use crate::applib::errors::AppError;
use crate::db::model::destination::{Destination, NewDestination, UpdateDestination};
use crate::db::schema::destinations;
use crate::db::schema::destinations::dsl::*;
use crate::db::DbPool;
use actix_web::web;
use diesel::prelude::*;

pub async fn admin_get_all_destinations(
  db_pool: web::Data<DbPool>,
) -> Result<Vec<Destination>, AppError> {
  Ok(
    web::block(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      destinations
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??,
  )
}

pub async fn admin_get_all_unreviewed_destinations(
  db_pool: web::Data<DbPool>,
) -> Result<Vec<Destination>, AppError> {
  Ok(
    web::block(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;
      destinations
        .filter(isReviewed.eq(false))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??,
  )
}

pub async fn admin_get_destination_by_id(
  db_pool: web::Data<DbPool>,
  destination_id: u64,
) -> Result<Destination, AppError> {
  web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;
    destinations
      .filter(id.eq(destination_id))
      .limit(1)
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await??
  .pop()
  .map_or(
    Err(AppError::not_found(Some(String::from("destination")))),
    |destination| Ok(destination),
  )
}

pub async fn admin_create_destination(
  db_pool: web::Data<DbPool>,
  destination_json: web::Json<NewDestination>,
) -> Result<(), AppError> {
  let new_destination: NewDestination = destination_json.into_inner();

  web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    diesel::insert_into(destinations::table)
      .values(&new_destination)
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())
  })
  .await??;

  Ok(())
}

pub async fn admin_update_destination_by_id(
  db_pool: web::Data<DbPool>,
  destination_id: u64,
  update_destination_json: web::Json<UpdateDestination>,
) -> Result<(), AppError> {
  let update_destination: UpdateDestination = update_destination_json.into_inner();

  web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    diesel::update(destinations.filter(id.eq(destination_id)))
      .set(update_destination)
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())
  })
  .await??;

  Ok(())
}

pub async fn admin_delete_destination_by_id(
  db_pool: web::Data<DbPool>,
  destination_id: u64,
) -> Result<(), AppError> {
  web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    diesel::delete(destinations.filter(id.eq(destination_id)))
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())
  })
  .await??;
  Ok(())
}
