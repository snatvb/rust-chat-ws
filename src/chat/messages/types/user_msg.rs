
use serde::{Serialize, Deserialize};
use super::response_user_msg::ResponseUserMsg;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct UserMsg {
  pub receiver_id: u32,
  pub text: String,
}

impl UserMsg {
    pub fn to_response(&self, sender_id: u32) -> ResponseUserMsg {
      ResponseUserMsg {
        sender_id,
        text: self.text.clone(),
      }
    }
}