use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct ResponseUserMsg {
  pub sender_id: u32,
  pub text: String,
}
