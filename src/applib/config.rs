use std::env;

#[derive(Clone)]
pub struct Config {
  database_url: String,
  port: u16,
  jwt_secret: String,
}

impl Config {
  pub fn new() -> Self {
    Self {
      database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
      port: env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse::<u16>()
        .expect("Invalid port"),
      jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
    }
  }

  pub fn database_url(&self) -> String {
    self.database_url.clone()
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub fn jwt_secret(&self) -> String {
    self.jwt_secret.clone()
  }
}
