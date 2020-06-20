use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateDialog {
  with_id: u32,
}
