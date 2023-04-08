pub mod schema;
use r2d2;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
