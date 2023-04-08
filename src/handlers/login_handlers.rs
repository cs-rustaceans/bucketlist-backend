use crate::db::DbPool;
use crate::model::login_form::LoginForm;
use crate::service::login_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn login(
    pool: web::Data<DbPool>,
    login_form_json: web::Json<LoginForm>,
) -> Result<String, actix_web::error::Error> {
    login_service::login(pool, login_form_json)
        .await
        .map_err(|error| actix_web::error::ErrorImATeapot(error))
}
