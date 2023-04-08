mod model;
mod service;
mod db;
mod handlers;
use actix_web::{web, HttpServer, App};
use r2d2;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use std::env;
use crate::handlers::user_handlers;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv().ok();
    let pool = r2d2 ::Pool::builder()
        .build(
            ConnectionManager::<MysqlConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL should be set"))
        )
        .expect("Unexpected error getting a pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/admin/users")
                .service(web::resource("/")
                         .route(web::get().to(user_handlers::get_all_users))
                         .route(web::post().to(user_handlers::create_user))
                )
                .service(web::resource("/{id}")
                         .route(web::get().to(user_handlers::get_user))
                         .route(web::patch().to(user_handlers::update_user))
                         .route(web::delete().to(user_handlers::delete_user))
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
