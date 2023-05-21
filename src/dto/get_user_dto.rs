use serde::{Deserialize, Serialize};

use crate::db::model::user::User;

#[derive(Serialize, Deserialize)]
pub struct GetUserDTO {
  pub id: u64,
  pub role: String,
  pub status: String,
  pub email: String,
}

impl From<User> for GetUserDTO {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      role: user.role,
      status: user.status,
      email: user.email,
    }
  }
}
