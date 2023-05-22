use crate::db::schema::destinations;
use diesel::{AsChangeset, Insertable, Queryable};
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
  #[diesel(column_name = "ownerId")]
  pub owner_id: u64,
  pub visibility: String,
  #[diesel(column_name = "isReviewed")]
  pub is_reviewed: bool,
  pub name: String,
  pub latitude: f64,
  pub longitude: f64,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = destinations)]
pub struct UpdateDestination {
  #[diesel(column_name = "ownerId")]
  pub owner_id: Option<u64>,
  pub visibility: Option<String>,
  #[diesel(column_name = "isReviewed")]
  pub is_reviewed: Option<bool>,
  pub name: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
}
