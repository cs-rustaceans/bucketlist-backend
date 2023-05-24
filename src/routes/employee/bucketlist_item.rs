use crate::applib::errors::AppError;
use crate::db::model::bucketlist_item::NewBucketlistItem;
use crate::db::model::user::User;
use crate::db::DbPool;
use crate::service::employee::bucketlist_service;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;

async fn get_own_bucketlist(pool: web::Data<DbPool>, user: User) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(bucketlist_service::employee_get_own_bucketlist(pool, user).await?),
  )
}

async fn get_bucketlist_item_by_id(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  Ok(
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .json(bucketlist_service::employee_get_bucketlist_item_by_id(pool, user, *id).await?),
  )
}

async fn add_bucketlist_item_from_available_destination(
  pool: web::Data<DbPool>,
  user: User,
  new_bucketlist_item_json: web::Json<NewBucketlistItem>,
) -> Result<HttpResponse, AppError> {
  bucketlist_service::employee_add_bucketlist_item_from_available_destination(
    pool,
    user,
    new_bucketlist_item_json,
  )
  .await?;
  Ok(HttpResponse::Ok().into())
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("").route(web::get().to(get_own_bucketlist)))
    .service(
      web::scope("/add").service(
        web::resource("/from-available")
          .route(web::post().to(add_bucketlist_item_from_available_destination)),
      ),
    )
    .service(
      web::scope("/{id}")
        .service(web::resource("").route(web::get().to(get_bucketlist_item_by_id))),
    );
}
