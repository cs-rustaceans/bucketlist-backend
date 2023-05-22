use chrono::prelude::*;
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginTokenClaims {
  pub user_id: u64,
  pub session_id: u64,
  pub role: String,
  #[serde(with = "ts_seconds")]
  pub iat: DateTime<Utc>,
  #[serde(with = "ts_seconds")]
  pub exp: DateTime<Utc>,
}
