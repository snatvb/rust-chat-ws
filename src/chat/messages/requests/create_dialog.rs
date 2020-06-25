use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateDialog {
  pub nickname: String,
}
