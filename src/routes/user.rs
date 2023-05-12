use crate::db::DbPool;
use crate::db::model::user::{NewUser, User};
use crate::service::user_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn get_all_users(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::error::Error> {
    user_service::get_all_users(pool)
        .await
        .map(|users| {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&users).unwrap())
        })
        .map_err(|error| actix_web::error::ErrorImATeapot(error))
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<u64>,
) -> Result<User, actix_web::error::Error> {
    user_service::get_user_by_id(pool, *user_id)
        .await
        .map_err(|error| actix_web::error::ErrorImATeapot(error))
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<u64>,
) -> Result<String, actix_web::error::Error> {
    user_service::delete_user(pool, *user_id)
        .await
        .map(|_| String::from("User deleted"))
        .map_err(|error| actix_web::error::ErrorImATeapot(error))
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<u64>,
    user_json: web::Json<NewUser>,
) -> Result<String, actix_web::error::Error> {
    user_service::update_user(pool, *user_id, user_json)
        .await
        .map(|_| String::from("User updated"))
        .map_err(|error| actix_web::error::ErrorImATeapot(error))
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    user_json: web::Json<NewUser>,
) -> Result<String, actix_web::error::Error> {
    user_service::create_user(pool, user_json)
        .await
        .map(|_| String::from("User created"))
        .map_err(|error| actix_web::error::ErrorImATeapot(error))
}
