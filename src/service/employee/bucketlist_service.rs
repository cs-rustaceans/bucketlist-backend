use crate::applib::errors::AppError;
use crate::db::model::bucketlist_item::BucketlistItem;
use crate::db::model::bucketlist_item::NewBucketlistItem;
use crate::db::model::destination::Destination;
use crate::db::model::user::User;
use crate::db::predicates::destination::available_for_user;
use crate::db::schema::bucketlist_items;
use crate::db::schema::destinations;
use crate::db::DbPool;
use crate::dto::bucketlist_item_with_destination_dto::BucketlistItemWithDestinationDTO;
use actix_web::web;
use diesel::prelude::*;

pub async fn employee_get_own_bucketlist(
  db_pool: web::Data<DbPool>,
  user: User,
) -> Result<Vec<BucketlistItemWithDestinationDTO>, AppError> {
  Ok(
    web::block(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      bucketlist_items::dsl::bucketlist_items
        .filter(bucketlist_items::dsl::ownerId.eq(user.id))
        .inner_join(destinations::table)
        .select((BucketlistItem::as_select(), Destination::as_select()))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??
    .into_iter()
    .map(|(bucketlist_item, destination)| BucketlistItemWithDestinationDTO{bucketlist_item, destination})
    .collect()
  )
}

pub async fn employee_get_bucketlist_item_by_id(
  db_pool: web::Data<DbPool>,
  user: User,
  bucketlist_item_id: u64,
) -> Result<BucketlistItemWithDestinationDTO, AppError> {
  Ok(
    web::block::<_, Result<Vec<(BucketlistItem, Destination)>, AppError>>(move || {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;

      bucketlist_items::dsl::bucketlist_items
        .filter(
          bucketlist_items::dsl::ownerId
            .eq(user.id)
            .and(bucketlist_items::dsl::id.eq(bucketlist_item_id)),
        )
        .inner_join(destinations::table)
        .select((BucketlistItem::as_select(), Destination::as_select()))
        .load(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??
    .pop()
    .ok_or(AppError::not_found(Some(String::from("bucketlist item"))))
    .map(|(bucketlist_item, destination)| BucketlistItemWithDestinationDTO{bucketlist_item, destination})?
  )
}

pub async fn employee_add_bucketlist_item_from_available_destination(
  db_pool: web::Data<DbPool>,
  user: User,
  new_bucketlist_item_json: web::Json<NewBucketlistItem>,
) -> Result<(), AppError> {
  if new_bucketlist_item_json.start_date > new_bucketlist_item_json.end_date {
    return Err(AppError::bad_request());
  }

  let mut new_bucketlist_item = new_bucketlist_item_json.into_inner();

  new_bucketlist_item.owner_id = Some(user.id);

  web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;
    db_connection.transaction::<_, AppError, _>(|db_connection| {
      let result: Vec<Destination> = destinations::dsl::destinations
        .filter(
          available_for_user(user.id)
            .and(destinations::dsl::id.eq(new_bucketlist_item.destination_id)),
        )
        .load(db_connection)
        .map_err(|_| AppError::internal_server_error())?;
      if result.len() != 1 {
        return Err(AppError::not_found(Some(String::from("destination"))));
      }

      diesel::insert_into(bucketlist_items::table)
        .values(new_bucketlist_item)
        .execute(db_connection)?;

      Ok(())
    })
  })
  .await?
}
