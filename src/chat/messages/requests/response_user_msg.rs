use serde::{Serialize, Deserialize};
use crate::id_record;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct ResponseUserMsg {
  pub sender_id: id_record::KeyType,
  pub text: String,
}
