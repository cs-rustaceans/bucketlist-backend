use crate::applib::errors::AppError;
use crate::db::model::user::User;
use crate::db::schema::sessions;
use crate::db::schema::users;
use crate::db::{model::user::UpdateUser, DbPool};
use crate::dto::user_change_password_dto::UserChangePasswordDTO;
use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;

pub async fn employee_change_password(
  db_pool: web::Data<DbPool>,
  user: User,
  change_password_json: web::Json<UserChangePasswordDTO>,
) -> Result<(), AppError> {
  let change_password_dto: UserChangePasswordDTO = change_password_json.into_inner();

  if !verify(change_password_dto.old_password, user.password.as_str())
    .map_err(|_| AppError::internal_server_error())?
  {
    return Err(AppError::invalid_email_password());
  }

  let update_user: UpdateUser = UpdateUser {
    email: None,
    role: None,
    status: None,
    password: Some(
      hash(change_password_dto.new_password, DEFAULT_COST)
        .map_err(|_| AppError::internal_server_error())?,
    ),
  };

  let row_count: usize = web::block(move || {
    let mut db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    db_connection.transaction::<_, AppError, _>(|db_connection| {
      diesel::delete(sessions::table)
        .filter(sessions::dsl::userId.eq(user.id))
        .execute(db_connection)?;
      diesel::update(users::table)
        .filter(users::dsl::id.eq(user.id))
        .set(update_user)
        .execute(db_connection)
        .map_err(|_| AppError::internal_server_error())
    })
  })
  .await??;

  // Not sure how this would happend
  if row_count == 0 {
    Err(AppError::not_found(Some(String::from("User"))))
  } else {
    Ok(())
  }
}
