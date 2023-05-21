use crate::db::schema::destinations;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = destinations)]
pub struct Destination {
  pub id: u64,
  pub owner_id: u64,
  pub visiblity: String,
  pub is_reviewed: bool,
  pub name: String,
  pub latitude: f64,
  pub longitude: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = destinations)]
pub struct NewDestination {
  #[column_name = "ownerId"]
  pub owner_id: u64,
  pub visibility: String,
  #[column_name = "isReviewed"]
  pub is_reviewed: bool,
  pub name: String,
  pub latitude: f64,
  pub longitude: f64,
}
