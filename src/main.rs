mod applib;
mod db;
mod dto;
mod routes;
mod service;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use applib::config::Config;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().expect("Could not load configuration");

  let config = Config::new();

  let pool = r2d2::Pool::builder()
    .build(ConnectionManager::<MysqlConnection>::new(
      config.database_url(),
    ))
    .expect("Unexpected error getting a pool");

  HttpServer::new(move || {
    let cors = Cors::permissive();
    App::new()
      .wrap(cors)
      .app_data(web::Data::new(pool.clone()))
      .configure(routes::configure_routes)
  })
  .bind(("127.0.0.1", config.port()))?
  .run()
  .await
}
