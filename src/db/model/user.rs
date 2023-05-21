use crate::db::schema::users;
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: u64,
  pub role: String,
  pub status: String,
  pub email: String,
  pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub role: String,
  pub email: String,
  pub password: String,
}

impl Responder for User {
  type Body = BoxBody;
  fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .body(serde_json::to_string(&self).unwrap())
  }
}
