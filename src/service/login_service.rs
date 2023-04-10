use crate::db::schema::users::dsl::*;
use crate::db::DbPool;
use crate::dto::login_form::LoginForm;
use crate::model::user::User;
use actix_web::web;
use bcrypt::verify;
use diesel::prelude::*;

pub async fn login(
    db_pool: web::Data<DbPool>,
    login_form_json: web::Json<LoginForm>,
) -> Result<String, String> {
    let login_form: LoginForm = login_form_json.into_inner();

    let result = web::block(move || {
        let mut db_connection;

        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(String::from("error getting db connection"));
        }
        let db_users: Vec<User> = users
            .filter(email.like(format!("%{}%", login_form.email)))
            .limit(1)
            .load(&mut db_connection)
            .map_err(|error| error.to_string())?;

        if db_users.len() != 1 {
            return Err(String::from("User with such email not found"));
        }

        if verify(&login_form.password, &db_users[0].password).unwrap() {
            return Ok(db_users[0].role.to_owned());
        } else {
            return Err(String::from("Login failed"));
        }
    })
    .await;

    match result {
        Ok(inner_result) => inner_result.map_err(|error| error.to_string()),
        Err(error) => Err(error.to_string()),
    }
}
