use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
  id: u32,
  name: String,
}
