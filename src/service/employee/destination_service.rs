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
