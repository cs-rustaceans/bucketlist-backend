use diesel::prelude::*;
use crate::model::user::{NewUser, User};
use actix_web::web;
use bcrypt::{hash, DEFAULT_COST};
use crate::db::DbPool;
use crate::db::schema::users;
use crate::db::schema::users::dsl::*;

pub async fn create_user(db_pool: web::Data<DbPool>, user_json: web::Json<NewUser>) -> Result< (), String > {
    let mut new_user: NewUser = user_json.into_inner();
    
    if let Ok(hashed_password) = hash(new_user.password, DEFAULT_COST) {
        new_user.password = hashed_password; 
    } else {
        return Err(String::from("Error encrypting password"));
    }

    let result = web::block(move || {
        let mut db_connection;
    
        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(String::from("error getting db connection"));
        }
    
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut db_connection)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
    .await;

    match result {
        Ok(inner_result) => inner_result.map_err(|error| error.to_string()),
        Err(error) => Err(error.to_string())
    }
}

pub async fn get_all_users(db_pool: web::Data<DbPool>) -> Result< Vec<User>, String > {
    let result = web::block(move || {
        let mut db_connection;
    
        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(String::from("error getting db connection"));
        }
        users
            .load(&mut db_connection)
            .map_err(|error| error.to_string())
    })
    .await;
    
    match result {
        Ok(inner_result) => inner_result.map_err(|error| error.to_string()),
        Err(error) => Err(error.to_string())
    }
}

pub async fn get_user_by_id(db_pool: web::Data<DbPool>, user_id: u64) -> Result< User, String > {
    let result = web::block(move || {
        let mut db_connection;
    
        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(String::from("error getting db connection"));
        }
        users
            .filter(id.eq(user_id))
            .limit(1)
            .load(&mut db_connection)
            .map_err(|error| error.to_string())
    })
    .await;

    match result {
        Ok(Ok(mut result_users)) => result_users.pop().map_or(Err(String::from("No such user found")), |user| Ok(user)),
        Ok(Err(inner_error)) => Err(inner_error.to_string()),
        Err(error) => Err(error.to_string())
    }
}

pub async fn update_user(db_pool: web::Data<DbPool>, user_id: u64, user_json: web::Json<NewUser> ) -> Result< (), String > {
    let new_user: NewUser = user_json.into_inner();

    let result = web::block(move || {
        let mut db_connection;
    
        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(String::from("error getting db connection"));
        }
        
        diesel::update(users.filter(id.eq(user_id)))
            .set(new_user)
            .execute(&mut db_connection)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
    .await;
    
    match result {
        Ok(inner_result) => inner_result.map_err(|error| error.to_string()),
        Err(error) => Err(error.to_string())
    }
}

pub async fn delete_user(db_pool: web::Data<DbPool>, user_id: u64 ) -> Result< (), String > {
    let result = web::block(move || {
        let mut db_connection;
    
        if let Ok(connection) = db_pool.get() {
            db_connection = connection;
        } else {
            return Err(String::from("error getting db connection"));
        }
    
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(&mut db_connection)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
    .await;

    match result {
        Ok(inner_result) => inner_result.map_err(|error| error.to_string()),
        Err(error) => Err(error.to_string())
    }
}
