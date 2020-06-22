use super::response_user_msg::ResponseUserMsg;
use crate::id_record;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct UserMsg {
    pub receiver_id: id_record::KeyType,
    pub text: String,
}

impl UserMsg {
    pub fn to_response(&self, sender_id: id_record::KeyType) -> ResponseUserMsg {
        ResponseUserMsg {
            sender_id,
            text: self.text.clone(),
        }
    }
}
