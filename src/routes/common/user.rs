use crate::applib::errors::AppError;
use crate::db::model::user::User;
use crate::dto::get_user_dto::GetUserDTO;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn get_user(user: User) -> Result<HttpResponse, AppError> {
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(GetUserDTO::from(user)),
  );
}
