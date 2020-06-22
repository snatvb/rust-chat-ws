use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Code {
  Forbidden = 403,
  NotFound = 404,
  BadRequest = 400,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
  pub code: Code,
  pub message: String,
}

pub fn forbidden(msg: &str) -> Error {
  Error {
    code: Code::Forbidden,
    message: msg.to_string(),
  }
}

pub fn not_found(msg: &str) -> Error {
  Error {
    code: Code::NotFound,
    message: msg.to_string(),
  }
}

pub fn bad_request(msg: &str) -> Error {
  Error {
    code: Code::BadRequest,
    message: msg.to_string(),
  }
}
