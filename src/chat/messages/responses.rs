pub mod registered;
pub mod error;
pub mod ok;

use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
  OK(ok::OK),
  Error(error::Error),
  Registered(registered::Registered),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
  Registered = 0,
  Error = 13,
  Message = 1,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Packet {
  pub action_type: ActionType,
  pub payload: Payload,
}
