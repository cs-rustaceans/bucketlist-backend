use crate::{applib::errors::AppError, db::schema::users};
use actix_web::{FromRequest, HttpMessage};
use diesel::prelude::*;
use futures_util;
use futures_util::future::{err, ok};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Clone)]
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
  pub password: Option<String>,
  pub status: Option<String>,
}

pub enum RoleEnum {
  Admin,
  Employee,
}

pub enum StatusEnum {
  Active,
  Inactive,
  Deleted,
}

impl FromRequest for User {
  type Error = AppError;
  type Future = futures_util::future::Ready<Result<Self, Self::Error>>;

  fn from_request(
    req: &actix_web::HttpRequest,
    _playload: &mut actix_web::dev::Payload,
  ) -> Self::Future {
    match req.extensions().get::<User>() {
      Some(user) => ok(user.clone()),
      None => err(AppError::unauthorized()),
    }
  }
}

impl Into<&str> for RoleEnum {
  fn into(self: Self) -> &'static str {
    match self {
      Self::Admin => "admin",
      Self::Employee => "employee",
    }
  }
}

impl TryFrom<&str> for RoleEnum {
  type Error = AppError;
  fn try_from(s: &str) -> Result<RoleEnum, Self::Error> {
    match s {
      "admin" => Ok(Self::Admin),
      "employee" => Ok(Self::Employee),
      _ => Err(AppError::bad_request()),
    }
  }
}

impl Into<&str> for StatusEnum {
  fn into(self: Self) -> &'static str {
    match self {
      Self::Active => "active",
      Self::Inactive => "inactive",
      Self::Deleted => "deleted",
    }
  }
}

impl TryFrom<&str> for StatusEnum {
  type Error = AppError;
  fn try_from(s: &str) -> Result<StatusEnum, Self::Error> {
    match s {
      "active" => Ok(Self::Active),
      "inactive" => Ok(Self::Inactive),
      "deleted" => Ok(Self::Deleted),
      _ => Err(AppError::bad_request()),
    }
  }
}
