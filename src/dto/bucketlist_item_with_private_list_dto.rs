use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BucketlistItemWithPrivateListDTO {
  pub name: String,
  pub latitude: f64,
  pub longitude: f64,
  pub start_date: NaiveDateTime,
  pub end_date: NaiveDateTime,
}
