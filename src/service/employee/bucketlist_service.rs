use crate::applib::errors::AppError;
use crate::db::model::bucketlist_item::BucketlistItem;
use crate::db::model::user::User;
use crate::db::schema::bucketlist_items::dsl::*;
use crate::db::DbPool;
use actix_web::web;
use diesel::prelude::*;

pub async fn employee_get_own_bucketlist(
  db_pool: web::Data<DbPool>,
  user: User,
) -> Result<Vec<BucketlistItem>, AppError> {
  Ok(
    web::block(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      bucketlist_items
        .filter(ownerId.eq(user.id))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??,
  )
}
