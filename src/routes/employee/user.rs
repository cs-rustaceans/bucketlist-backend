use crate::applib::errors::AppError;
use crate::db::model::user::User;
use crate::db::DbPool;
use crate::dto::user_change_password_dto::UserChangePasswordDTO;
use crate::routes::common;
use crate::service::employee::user_service;
use actix_web::{web, HttpResponse};

async fn employee_change_password(
  db_pool: web::Data<DbPool>,
  user: User,
  change_password_json: web::Json<UserChangePasswordDTO>,
) -> Result<HttpResponse, AppError> {
  user_service::employee_change_password(db_pool, user, change_password_json).await?;
  Ok(HttpResponse::Ok().into())
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("/me").route(web::get().to(common::user::get_user)))
    .service(web::resource("/change-password").route(web::post().to(employee_change_password)));
}
