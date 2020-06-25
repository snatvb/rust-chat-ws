use serde::{Serialize, Deserialize};
use crate::chat::store::dialogs;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct CreatedDialog {
  pub dialog: dialogs::Dialog,
}
