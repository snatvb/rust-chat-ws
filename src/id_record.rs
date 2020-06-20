use core::ops::Add;
use std::collections::{HashMap};

pub type IDControlItem<U, T> = (U, T);

pub trait IDControl<U, T> {
  fn save(&mut self, value: T) -> U;
  fn remove(&mut self, id: &U) -> Option<IDControlItem<U, T>>;
  fn get(&self, id: &U) -> Option<&IDControlItem<U, T>>;
}

pub struct IDRecord<U: Add, T> {
  items: HashMap<U, IDControlItem<U, T>>,
  last_id: U,
}

impl<T> IDRecord<u32, T> {
  pub fn new() -> Self {
    IDRecord {
      items: HashMap::new(),
      last_id: 0,
    }
  }
}

impl<T> IDRecord<u32, T> {
  pub fn iter(&self) -> impl Iterator<Item = (&u32, &IDControlItem<u32, T>)> {
    self.items.iter()
  }
}


impl<T> IDControl<u32, T> for IDRecord<u32, T> {
    fn save(&mut self, value: T) -> u32 {
        self.last_id += 1;
        let item = (self.last_id, value);
        self.items.insert(self.last_id, item);
        self.last_id
    }

    fn remove(&mut self, id: &u32) -> Option<IDControlItem<u32, T>> {
        self.items.remove(id)
    }

    fn get(&self, id: &u32) -> Option<&IDControlItem<u32, T>> {
        self.items.get(id)
    }
}
