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

pub async fn employee_get_bucketlist_item_by_id(
  db_pool: web::Data<DbPool>,
  user: User,
  bucketlist_item_id: u64,
) -> Result<BucketlistItem, AppError> {
  Ok(
    web::block::<_, Result<Vec<BucketlistItem>, AppError>>(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      bucketlist_items
        .filter(ownerId.eq(user.id).and(id.eq(bucketlist_item_id)))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??
    .pop()
    .ok_or(AppError::not_found(Some(String::from("bucketlist item"))))?,
  )
}
