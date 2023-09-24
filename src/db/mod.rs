pub mod model;
pub mod predicates;
pub mod schema;
use diesel::mysql::Mysql;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;

pub type Db = Mysql;
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

use diesel::sql_function;
use diesel::sql_types;
sql_function! {
 fn last_insert_id() -> sql_types::BigInt;
}
