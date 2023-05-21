use diesel::Queryable;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = destinations)]
pub struct Destination {
    pub id: u64,
    pub owner_id: u64,
    pub visiblity: String,
    pub is_reviewed: bool,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64
}
