use crate::db::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: u64,
  pub role: String,
  pub email: String,
  pub password: String,
  pub status: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub role: String,
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
  pub role: Option<String>,
  pub email: Option<String>,
  pub status: Option<String>,
  pub password: Option<String>,
}
