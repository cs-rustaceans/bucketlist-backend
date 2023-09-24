use crate::applib::errors::AppError;
use crate::db::model::bucketlist_item::BucketlistItem;
use crate::db::model::destination::Destination;
use crate::db::model::user::StatusEnum;
use crate::db::schema::bucketlist_items;
use crate::db::schema::destinations;
use crate::db::schema::users;
use crate::db::DbPool;
use crate::dto::bucketlist_item_with_destination_dto::BucketlistItemWithDestinationDTO;
use actix_web::web;
use diesel::prelude::*;
use std::collections::HashMap;

pub async fn admin_get_favorite_bucketlist_items(
  db_pool: web::Data<DbPool>,
) -> Result<Vec<BucketlistItemWithDestinationDTO>, AppError> {
  let mut choices: HashMap<u64, BucketlistItemWithDestinationDTO> = HashMap::new();

  let result = web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    bucketlist_items::table
      .inner_join(users::table)
      .inner_join(destinations::table)
      .filter(users::dsl::status.eq(Into::<&str>::into(StatusEnum::Active)))
      .order_by(bucketlist_items::dsl::isFavorite.asc())
      .select((BucketlistItem::as_select(), Destination::as_select()))
      .load::<(BucketlistItem, Destination)>(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await??;

  for (bucketlist_item, destination) in result {
    choices.insert(
      bucketlist_item.owner_id,
      BucketlistItemWithDestinationDTO {
        bucketlist_item,
        destination,
      },
    );
  }

  Ok(choices.into_iter().map(|(_, value)| value).collect())
}
