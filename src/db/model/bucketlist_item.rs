use crate::db::schema::bucketlist_items;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct BucketlistItem {
  pub id: u64,
  #[diesel(column_name = "destinationId")]
  pub destination_id: u64,
  #[diesel(column_name = "ownerId")]
  pub owner_id: u64,
  #[diesel(column_name = "startDate")]
  pub start_date: NaiveDateTime,
  #[diesel(column_name = "endDate")]
  pub end_date: NaiveDateTime,
  #[diesel(column_name = "isFavorite")]
  pub is_favorite: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=bucketlist_items)]
pub struct NewBucketlistItem {
  #[diesel(column_name = "destinationId")]
  pub destination_id: u64,
  #[diesel(column_name = "ownerId")]
  pub owner_id: Option<u64>,
  #[diesel(column_name = "startDate")]
  pub start_date: NaiveDateTime,
  #[diesel(column_name = "endDate")]
  pub end_date: NaiveDateTime,
}
