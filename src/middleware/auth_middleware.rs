use crate::db::model::session::Session;
use crate::db::model::user::User;
use crate::db::schema::sessions;
use crate::db::schema::users;
use crate::{
  applib::{config::Config, errors::AppError},
  db::DbPool,
  dto::login_token_claims::LoginTokenClaims,
};
use actix_web::dev::ServiceRequest;
use actix_web::error::Error;
use actix_web::web;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::prelude::*;
use diesel::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

pub async fn authentification_middleware(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
  let jwt_secret_option: Option<String> = req
    .app_data::<web::Data<Config>>()
    .cloned()
    .map(|config| config.jwt_secret());

  if jwt_secret_option.is_none() {
    return Err((AppError::internal_server_error().into(), req));
  }

  let claims_result: Result<LoginTokenClaims, _> = jsonwebtoken::decode::<LoginTokenClaims>(
    credentials.token(),
    &DecodingKey::from_secret(jwt_secret_option.unwrap().as_bytes()),
    &Validation::new(Algorithm::HS256),
  )
  .map(|token| token.claims);

  if claims_result.is_err() {
    return Err((AppError::internal_server_error().into(), req));
  }

  let claims = claims_result.unwrap();

  let db_pool_option: Option<web::Data<DbPool>> = req.app_data::<web::Data<DbPool>>().cloned();

  if db_pool_option.is_none() {
    return Err((AppError::internal_server_error().into(), req));
  }

  let user_result_result = web::block(move || -> Result<User, AppError> {
    let mut db_connection;

    db_connection = db_pool_option
      .unwrap()
      .get()
      .map_err(|_| AppError::internal_server_error())?;

    let session: Session = sessions::dsl::sessions
      .filter(sessions::dsl::id.eq(claims.session_id))
      .limit(1)
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())?
      .pop()
      .ok_or(AppError::unauthorized())?;

    if session.expires_at < Utc::now().naive_utc() {
      return Err(AppError::unauthorized());
    }

    let user: User = users::dsl::users
      .filter(users::dsl::id.eq(session.user_id))
      .limit(1)
      .load(&mut db_connection)
      .map_err(|_| AppError::internal_server_error())?
      .pop()
      .ok_or(AppError::unauthorized())?;

    return Ok(user);
  })
  .await;

  if let Err(_) = user_result_result {
    return Err((AppError::internal_server_error().into(), req));
  } else if let Ok(Err(_)) = user_result_result {
    return Err((AppError::unauthorized().into(), req));
  }
  let user: User = user_result_result.unwrap().unwrap();

  req.extensions_mut().insert(user);
  return Ok(req);
}
