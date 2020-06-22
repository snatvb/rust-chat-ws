use serde::{Serialize, Deserialize};
use crate::chat::store::User;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Registered {
  pub user: User,
}
