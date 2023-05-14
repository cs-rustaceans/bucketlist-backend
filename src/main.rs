mod db;
mod routes;
mod service;
mod dto;
mod applib;
use crate::routes::{login, user};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use applib::config::Config;
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
            .service(
                web::scope("/admin/users")
                    .service(
                        web::resource("")
                            .route(web::get().to(user::get_all_users))
                            .route(web::post().to(user::create_user)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(user::get_user))
                            .route(web::patch().to(user::update_user))
                            .route(web::delete().to(user::delete_user)),
                    ),
            )
            .service(
                web::scope("/login")
                    .service(web::resource("").route(web::post().to(login::login))),
            )
    })
    .bind((
        "127.0.0.1",
        config.port(),
    ))?
    .run()
    .await
}
