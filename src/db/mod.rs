pub mod schema;
pub mod model;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
