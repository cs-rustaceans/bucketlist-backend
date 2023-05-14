use crate::db::model::user::{NewUser, User};
use crate::db::DbPool;
use crate::service::user_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn get_all_users(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, impl actix_web::ResponseError> {
    user_service::get_all_users(pool).await.map(|users| {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&users).unwrap())
    })
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<u64>,
) -> Result<User, impl actix_web::ResponseError> {
    user_service::get_user_by_id(pool, *user_id).await
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<u64>,
) -> Result<String, impl actix_web::ResponseError> {
    user_service::delete_user(pool, *user_id)
        .await
        .map(|_| String::from("User deleted"))
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<u64>,
    user_json: web::Json<NewUser>,
) -> Result<String, impl actix_web::ResponseError> {
    user_service::update_user(pool, *user_id, user_json)
        .await
        .map(|_| String::from("User updated"))
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    user_json: web::Json<NewUser>,
) -> Result<String, impl actix_web::ResponseError> {
    user_service::create_user(pool, user_json)
        .await
        .map(|_| String::from("User created"))
}
