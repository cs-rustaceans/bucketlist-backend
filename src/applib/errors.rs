use actix_web::{
  error::{self, BlockingError},
  http::{header::ContentType, StatusCode},
  HttpResponse,
};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct AppError {
  #[serde(skip_serializing)]
  pub status: StatusCode,
  pub code: String,
  pub message: String,
}

impl AppError {
  // fn new(status: StatusCode, code: String, message: String) -> Self {
  //   Self {
  //     status,
  //     code,
  //     message,
  //   }
  // }

  pub fn internal_server_error() -> Self {
    Self {
      status: StatusCode::INTERNAL_SERVER_ERROR,
      code: "InternalServerError".to_string(),
      message: "A server-side error occurred.".to_string(),
    }
  }

  pub fn not_found(entity: Option<String>) -> Self {
    Self {
      status: StatusCode::NOT_FOUND,
      code: "NotFound".to_string(),
      message: format!(
        "The requested entity {} was not found.",
        entity
          .map(|o| " ".to_owned() + &o)
          .unwrap_or("".to_string())
      ),
    }
  }

  pub fn bad_request() -> Self {
    Self {
      status: StatusCode::BAD_REQUEST,
      code: "Bad request".to_string(),
      message: "Bad request.".to_string(),
    }
  }

  // LOGIN
  pub fn invalid_email_password() -> Self {
    Self {
      status: StatusCode::BAD_REQUEST,
      code: "InvalidEmailPassword".to_string(),
      message: "The email or password are invalid.".to_string(),
    }
  }

  pub fn unauthorized() -> Self {
    Self {
      status: StatusCode::UNAUTHORIZED,
      code: "Unauthorized".to_string(),
      message: "Unauthorized to access resource.".to_string(),
    }
  }

  pub fn user_inactive_or_deleted() -> Self {
    Self {
      status: StatusCode::UNAUTHORIZED,
      code: "UserInactiveOrDeleted".to_string(),
      message: "User account is not active or is deleted".to_string(),
    }
  }

  // bucketlist items

  pub fn destination_is_not_reviewed() -> Self {
    Self {
      status: StatusCode::BAD_REQUEST,
      code: "Bad request".to_string(),
      message: "The destination of this bucketlist has not yet been verified by an admin"
        .to_string(),
    }
  }
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "HTTP {} {}: {}", self.status, self.code, self.message)
  }
}

impl error::ResponseError for AppError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    self.status
  }

  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::json())
      .body(serde_json::to_string(self).expect(""))
  }
}

impl From<BlockingError> for AppError {
  fn from(_: BlockingError) -> AppError {
    AppError::internal_server_error()
  }
}

impl From<diesel::result::Error> for AppError {
  fn from(_: diesel::result::Error) -> AppError {
    AppError::internal_server_error()
  }
}
