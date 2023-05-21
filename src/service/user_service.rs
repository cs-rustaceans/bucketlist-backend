use crate::applib::errors::AppError;
use crate::db::model::user::{NewUser, UpdateUser, User};
use crate::db::schema::users;
use crate::db::schema::users::dsl::*;
use crate::db::DbPool;
use actix_web::web;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

pub async fn create_user(
  db_pool: web::Data<DbPool>,
  user_json: web::Json<NewUser>,
) -> Result<(), AppError> {
  let mut new_user: NewUser = user_json.into_inner();

  if let Ok(hashed_password) = hash(new_user.password, DEFAULT_COST) {
    new_user.password = hashed_password;
  } else {
    return Err(AppError::internal_server_error());
  }

  let result = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }

    diesel::insert_into(users::table)
      .values(&new_user)
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(inner_result) => inner_result,
    Err(_) => Err(AppError::internal_server_error()),
  }
}

pub async fn get_all_users(db_pool: web::Data<DbPool>) -> Result<Vec<User>, AppError> {
  let result = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }
    users
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(inner_result) => inner_result,
    Err(_) => Err(AppError::internal_server_error()),
  }
}

pub async fn get_user_by_id(db_pool: web::Data<DbPool>, user_id: u64) -> Result<User, AppError> {
  let result = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }
    users
      .filter(id.eq(user_id))
      .limit(1)
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(Ok(mut result_users)) => result_users.pop().map_or(
      Err(AppError::not_found(Some(String::from("user")))),
      |user| Ok(user),
    ),
    Ok(Err(inner_error)) => Err(inner_error),
    Err(_) => Err(AppError::internal_server_error()),
  }
}

pub async fn update_user(
  db_pool: web::Data<DbPool>,
  user_id: u64,
  user_json: web::Json<UpdateUser>,
) -> Result<(), AppError> {
  let mut update_user: UpdateUser = user_json.into_inner();

  if let Some(plain_text_password) = update_user.password {
    if let Ok(hashed_password) = hash(plain_text_password, DEFAULT_COST) {
      update_user.password = Some(hashed_password);
    } else {
      return Err(AppError::internal_server_error());
    }
  }

  let result = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }

    diesel::update(users.filter(id.eq(user_id)))
      .set(update_user)
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(inner_result) => inner_result,
    Err(_) => Err(AppError::internal_server_error()),
  }
}

pub async fn delete_user(db_pool: web::Data<DbPool>, user_id: u64) -> Result<(), AppError> {
  let result = web::block(move || {
    let mut db_connection;

    if let Ok(connection) = db_pool.get() {
      db_connection = connection;
    } else {
      return Err(AppError::internal_server_error());
    }

    diesel::delete(users.filter(id.eq(user_id)))
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())
  })
  .await;

  match result {
    Ok(inner_result) => inner_result,
    Err(_) => Err(AppError::internal_server_error()),
  }
}
