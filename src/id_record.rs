use core::ops::Add;
use std::collections::HashMap;

pub type KeyType = u64;

pub type IDControlItem<T> = (KeyType, T);

pub trait IDControl<T> {
    fn save(&mut self, value: T) -> KeyType;
    fn remove(&mut self, id: &KeyType) -> Option<IDControlItem<T>>;
    fn get(&self, id: &KeyType) -> Option<&IDControlItem<T>>;
    fn get_item(&self, id: &KeyType) -> Option<&T>;
    fn get_item_value(&self, id: &KeyType) -> Option<T>;
}

pub struct IDRecord<T> {
    items: HashMap<u64, IDControlItem<T>>,
    counter: IDCounter,
}

impl<T> IDRecord<T> {
    pub fn new() -> Self {
        IDRecord {
            items: HashMap::new(),
            counter: IDCounter::new(),
        }
    }
}

impl<T> IDRecord<T> {
    pub fn iter(&self) -> impl Iterator<Item = (&KeyType, &IDControlItem<T>)> {
        self.items.iter()
    }
}

impl<T: Clone> IDControl<T> for IDRecord<T> {
    fn save(&mut self, value: T) -> KeyType {
        let id = self.counter.get_new_id();
        let item = (id, value);
        self.items.insert(id, item);
        id
    }

    fn remove(&mut self, id: &KeyType) -> Option<IDControlItem<T>> {
        self.items.remove(id)
    }

    fn get(&self, id: &KeyType) -> Option<&IDControlItem<T>> {
        self.items.get(id)
    }

    fn get_item(&self, id: &KeyType) -> Option<&T> {
        self.items.get(id).map(|(_, x)| x)
    }

    fn get_item_value(&self, id: &KeyType) -> Option<T> {
        self.items.get(id).map(|(_, x)| x.clone())
    }
}


pub struct IDCounter {
    last_id: u64,
}

impl IDCounter {
    pub fn new() -> Self {
        IDCounter { last_id: 0 }
    }

    pub fn get_new_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }
}
