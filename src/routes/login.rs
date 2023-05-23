use crate::applib::config::Config;
use crate::db::DbPool;
use crate::dto::login_form::LoginForm;
use crate::service::login_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use std::collections::HashMap;

async fn login(
  pool: web::Data<DbPool>,
  config: web::Data<Config>,
  login_form_json: web::Json<LoginForm>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
  let result = login_service::login(pool, config, login_form_json).await;
  if let Err(error) = result {
    return Err(error);
  }
  let mut response: HashMap<String, String> = HashMap::new();
  response.insert(String::from("token"), result.unwrap());
  return Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(response),
  );
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("").route(web::post().to(login)));
}
