use crate::applib::config::Config;
use crate::applib::errors::AppError;
use crate::db::last_insert_id;
use crate::db::model::session::NewSession;
use crate::db::model::session::Session;
use crate::db::model::user::User;
use crate::db::schema::sessions;
use crate::db::schema::users;
use crate::db::DbPool;
use crate::dto::login_form::LoginForm;
use crate::dto::login_token_claims::LoginTokenClaims;
use actix_web::web;
use bcrypt::verify;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;

pub async fn login(
  db_pool: web::Data<DbPool>,
  config: web::Data<Config>,
  login_form_json: web::Json<LoginForm>,
) -> Result<String, AppError> {
  let login_form: LoginForm = login_form_json.into_inner();

  let db_pool_clone = db_pool.clone();
  let user: User = web::block(move || {
    let mut db_connection;

    db_connection = db_pool_clone
      .get()
      .map_err(|_| AppError::internal_server_error())?;
    let mut db_users: Vec<User> = users::dsl::users
      .filter(users::dsl::email.eq(login_form.email))
      .limit(1)
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())?;

    if db_users.len() != 1 {
      return Err(AppError::invalid_email_password());
    }

    if verify(&login_form.password, &db_users[0].password).unwrap() {
      return Ok(db_users.remove(0));
    } else {
      return Err(AppError::invalid_email_password());
    }
  })
  .await??;

  let new_session = NewSession {
    user_id: user.id,
    created_at: Utc::now().naive_utc(),
    expires_at: Utc::now()
      .checked_add_signed(chrono::Duration::weeks(1))
      .ok_or(AppError::internal_server_error())?
      .naive_utc(),
  };

  let session: Session = web::block(move || -> Result<Session, AppError> {
    let mut db_connection;

    db_connection = db_pool
      .get()
      .map_err(|_| AppError::internal_server_error())?;
    db_connection.transaction::<_, AppError, _>(|db_connection| {
      diesel::insert_into(sessions::table)
        .values(&new_session)
        .execute(db_connection)
        .map_err(|_| AppError::internal_server_error())?;
      let id: u64 = diesel::select(last_insert_id())
        .first::<i64>(db_connection)
        .map_err(|_| AppError::internal_server_error())? as u64;

      let session: Session = sessions::dsl::sessions
        .filter(sessions::dsl::id.eq(id))
        .limit(1)
        .load(db_connection)
        .map_err(|_| AppError::internal_server_error())?
        .pop()
        .ok_or(AppError::internal_server_error())?;
      return Ok(session);
    })
  })
  .await??;

  let login_token_claims = LoginTokenClaims {
    user_id: session.user_id,
    role: user.role,
    session_id: session.id,
    iat: DateTime::<Utc>::from_utc(session.created_at, Utc),
    exp: DateTime::<Utc>::from_utc(session.expires_at, Utc),
  };

  let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
  let token: String = jsonwebtoken::encode(
    &header,
    &login_token_claims,
    &jsonwebtoken::EncodingKey::from_secret(config.get_ref().jwt_secret().as_bytes()),
  )
  .map_err(|_| AppError::internal_server_error())?;

  return Ok(token);
}
