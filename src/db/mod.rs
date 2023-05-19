pub mod model;
pub mod schema;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
