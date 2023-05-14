use crate::db::DbPool;
use crate::dto::login_form::LoginForm;
use crate::service::login_service;
use actix_web::web;

pub async fn login(
    pool: web::Data<DbPool>,
    login_form_json: web::Json<LoginForm>,
) -> Result<String, impl actix_web::ResponseError> {
    login_service::login(pool, login_form_json).await
}
