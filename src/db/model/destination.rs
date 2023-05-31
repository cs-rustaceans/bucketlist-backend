use crate::applib::errors::AppError;
use crate::db::schema::destinations;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = destinations)]
pub struct Destination {
  pub id: u64,
  #[diesel(column_name = "ownerId")]
  pub owner_id: u64,
  pub visibility: String,
  #[diesel(column_name = "isReviewed")]
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

#[derive(Eq, PartialEq)]
pub enum VisibilityEnum {
  Public,
  Private,
}

impl Into<&str> for VisibilityEnum {
  fn into(self: Self) -> &'static str {
    match self {
      Self::Public => "public",
      Self::Private => "private",
    }
  }
}

impl TryFrom<&str> for VisibilityEnum {
  type Error = AppError;
  fn try_from(s: &str) -> Result<VisibilityEnum, Self::Error> {
    match s {
      "public" => Ok(Self::Public),
      "private" => Ok(Self::Private),
      _ => Err(AppError::bad_request()),
    }
  }
}
