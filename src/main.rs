mod db;
mod handlers;
mod model;
mod service;
use crate::handlers::{login_handlers, user_handlers};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<MysqlConnection>::new(
            env::var("DATABASE_URL").expect("DATABASE_URL should be set"),
        ))
        .expect("Unexpected error getting a pool");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/admin/users")
                    .service(
                        web::resource("")
                            .route(web::get().to(user_handlers::get_all_users))
                            .route(web::post().to(user_handlers::create_user)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(user_handlers::get_user))
                            .route(web::patch().to(user_handlers::update_user))
                            .route(web::delete().to(user_handlers::delete_user)),
                    ),
            )
            .service(
                web::scope("/login")
                    .service(web::resource("").route(web::get().to(login_handlers::login))),
            )
    })
    .bind((
        "127.0.0.1",
        env::var("PORT")
            .unwrap_or(String::from("8080"))
            .parse::<u16>()
            .expect("Error parsing port"),
    ))?
    .run()
    .await
}
