use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserChangePasswordDTO {
  pub old_password: String,
  pub new_password: String,
}
