use super::error;
use super::{error::Error, users};
use crate::id_record::IDCounter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Id = u64;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct Direct {
    pub id: Id,
    pub companion_id: users::Id,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Dialog {
    Direct(Direct),
}

pub struct Dialogs {
    counter: IDCounter,
    dialogs: HashMap<Id, Dialog>,
}

impl Dialogs {
    pub fn new() -> Self {
        Self {
            counter: IDCounter::new(),
            dialogs: HashMap::new(),
        }
    }

    pub fn create_direct(&mut self, companion_id: users::Id) -> error::Result<Dialog> {
        let id = self.counter.get_new_id();
        let dialog = Dialog::Direct(Direct { id, companion_id });
        self.dialogs
            .insert(id, dialog)
            .ok_or(Error::new("Insert was failed".to_string()))
    }
}
