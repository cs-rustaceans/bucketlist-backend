use crate::db::schema::sessions;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable)]
#[diesel(table_name=sessions)]
pub struct Session {
  pub id: u64,
  #[diesel(column_name = "userId")]
  pub user_id: u64,
  #[diesel(column_name = "createAt")]
  pub created_at: NaiveDateTime,
  #[diesel(column_name = "expiresAt")]
  pub expires_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=sessions)]
pub struct NewSession {
  #[diesel(column_name = "userId")]
  pub user_id: u64,
  #[diesel(column_name = "createdAt")]
  pub created_at: NaiveDateTime,
  #[diesel(column_name = "expiresAt")]
  pub expires_at: NaiveDateTime,
}
