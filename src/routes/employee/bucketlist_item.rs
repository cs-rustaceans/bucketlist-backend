use crate::applib::errors::AppError;
use crate::db::model::bucketlist_item::NewBucketlistItem;
use crate::db::model::bucketlist_item::UpdateBucketlistItem;
use crate::db::model::user::User;
use crate::db::DbPool;
use crate::dto::bucketlist_item_with_private_list_dto::BucketlistItemWithPrivateListDTO;
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
  Ok(HttpResponse::Created().into())
}

async fn add_bucketlist_item_with_private_list(
  pool: web::Data<DbPool>,
  user: User,
  new_bucketlist_item_json: web::Json<BucketlistItemWithPrivateListDTO>,
) -> Result<HttpResponse, AppError> {
  bucketlist_service::employee_add_bucketlist_item_with_private_list(
    pool,
    user,
    new_bucketlist_item_json,
  )
  .await?;
  Ok(HttpResponse::Created().into())
}

async fn update_bucketlist_item(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
  update_bucketlist_item_json: web::Json<UpdateBucketlistItem>,
) -> Result<HttpResponse, AppError> {
  bucketlist_service::employee_update_bucketlist_item(pool, user, *id, update_bucketlist_item_json)
    .await?;
  Ok(HttpResponse::Ok().into())
}

async fn delete_bucketlist_item(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  bucketlist_service::employee_delete_bucketlist_item(pool, user, *id).await?;
  Ok(HttpResponse::Ok().into())
}

async fn make_bucketlist_item_public(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  bucketlist_service::employee_make_bucketlist_item_destination_public(pool, user, *id).await?;
  Ok(HttpResponse::Ok().into())
}

async fn make_bucketlist_item_favorite(
  pool: web::Data<DbPool>,
  user: User,
  id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
  bucketlist_service::employee_make_bucketlist_item_destination_favorite(pool, user, *id).await?;
  Ok(HttpResponse::Ok().into())
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("").route(web::get().to(get_own_bucketlist)))
    .service(
      web::scope("/add")
        .service(
          web::resource("/from-available")
            .route(web::post().to(add_bucketlist_item_from_available_destination)),
        )
        .service(
          web::resource("/with-private-list")
            .route(web::post().to(add_bucketlist_item_with_private_list)),
        ),
    )
    .service(
      web::scope("/{id}")
        .service(
          web::resource("")
            .route(web::get().to(get_bucketlist_item_by_id))
            .route(web::patch().to(update_bucketlist_item))
            .route(web::delete().to(delete_bucketlist_item)),
        )
        .service(web::resource("/make-public").route(web::patch().to(make_bucketlist_item_public)))
        .service(
          web::resource("/make-favorite").route(web::patch().to(make_bucketlist_item_favorite)),
        ),
    );
}
