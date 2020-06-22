use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct Register {
  pub nickname: String,
  pub name: String,
  pub password: String,
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct Login {
  pub nickname: String,
  pub name: String,
}
