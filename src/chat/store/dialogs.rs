
use crate::id_record;

pub struct Direct {
  pub companion_id: id_record::KeyType,
}

pub enum Dialog {
  Direct(Direct),
}

pub type Dialogs = id_record::IDRecord<Dialog>;
