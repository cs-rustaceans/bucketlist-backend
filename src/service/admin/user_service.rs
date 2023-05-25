use crate::applib::errors::AppError;
use crate::db::model::user::{NewUser, UpdateUser, User};
use crate::db::schema::users;
use crate::db::schema::users::dsl::*;
use crate::db::DbPool;
use crate::dto::get_user_dto::GetUserDTO;
use actix_web::web;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

pub async fn admin_create_user(
  db_pool: web::Data<DbPool>,
  user_json: web::Json<NewUser>,
) -> Result<(), AppError> {
  let mut new_user: NewUser = user_json.into_inner();

  new_user.password =
    hash(new_user.password, DEFAULT_COST).map_err(|_| AppError::internal_server_error())?;

  web::block(move || -> Result<(), AppError> {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    diesel::insert_into(users::table)
      .values(&new_user)
      .execute(&mut db_connection)
      .map(|_| ())
      .map_err(|_| AppError::internal_server_error())?;
    Ok(())
  })
  .await??;

  Ok(())
}

pub async fn admin_get_all_users(db_pool: web::Data<DbPool>) -> Result<Vec<GetUserDTO>, AppError> {
  Ok(
    web::block(move || -> Result<Vec<User>, AppError> {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;
      users
        .load::<User>(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??
    .into_iter()
    .map(|user| GetUserDTO::from(user))
    .collect(),
  )
}

pub async fn admin_get_user_by_id(
  db_pool: web::Data<DbPool>,
  user_id: u64,
) -> Result<GetUserDTO, AppError> {
  Ok(
    web::block(move || -> Result<Vec<User>, AppError> {
      let mut db_connection = db_pool
        .get()
        .map_err(|_| AppError::internal_server_error())?;
      users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<User>(&mut db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
    .await??
    .pop()
    .map_or(
      Err(AppError::not_found(Some(String::from("user")))),
      |user| Ok(GetUserDTO::from(user)),
    )?,
  )
}

pub async fn admin_update_user(
  db_pool: web::Data<DbPool>,
  user_id: u64,
  user_json: web::Json<UpdateUser>,
) -> Result<(), AppError> {
  let mut update_user: UpdateUser = user_json.into_inner();

  if let Some(plain_text_password) = update_user.password {
    update_user.password =
      Some(hash(plain_text_password, DEFAULT_COST).map_err(|_| AppError::internal_server_error())?);
  }

  let row_count: usize = web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    diesel::update(users.filter(id.eq(user_id)))
      .set(update_user)
      .execute(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await??;

  if row_count == 0 {
    Err(AppError::not_found(Some(String::from("user"))))
  } else {
    Ok(())
  }
}

pub async fn admin_delete_user(db_pool: web::Data<DbPool>, user_id: u64) -> Result<(), AppError> {
  let row_count: usize = web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    diesel::delete(users.filter(id.eq(user_id)))
      .execute(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())
  })
  .await??;

  if row_count == 0 {
    Err(AppError::not_found(Some(String::from("user"))))
  } else {
    Ok(())
  }
}
