use crate::db::schema::users::dsl::*;
use crate::db::DbPool;
use crate::dto::login_form::LoginForm;
use crate::errors::AppError;
use crate::model::user::User;
use actix_web::web;
use bcrypt::verify;
use diesel::prelude::*;

pub async fn login(
    db_pool: web::Data<DbPool>,
    login_form_json: web::Json<LoginForm>,
) -> Result<String, AppError> {
    let login_form: LoginForm = login_form_json.into_inner();

    let result = web::block(move || {
        let mut db_connection;

        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(AppError::internal_server_error());
        }
        let db_users: Vec<User> = users
            .filter(email.like(format!("%{}%", login_form.email)))
            .limit(1)
            .load(&mut db_connection)
            .map_err(|_| AppError::internal_server_error())?;

        if db_users.len() != 1 {
            return Err(AppError::invalid_email_password());
        }

        if verify(&login_form.password, &db_users[0].password).unwrap() {
            return Ok(db_users[0].role.to_owned());
        } else {
            return Err(AppError::invalid_email_password());
        }
    })
    .await; 

    match result {
        Ok(inner_result) => inner_result.map_err(|error| error),
        Err(_) => Err(AppError::internal_server_error()),
    }
}
