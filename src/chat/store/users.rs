use crate::id_record;
use id_record::IDCounter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Id = id_record::KeyType;

pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub nickname: String,
    password: String,
}

pub struct Users {
    counter: IDCounter,
    users: HashMap<Id, User>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            counter: IDCounter::new(),
            users: HashMap::new(),
        }
    }
}

impl Users {
    pub fn add_user(&mut self, name: String, nickname: String, password: String) -> Result<User> {
        if self.get_by_nick(&nickname).is_some() {
            Err(Error::new(
                "User with such nickname already exists".to_string(),
            ))
        } else {
            let id = self.counter.get_new_id();
            let user = User {
                id,
                name,
                nickname,
                password,
            };
            self.users
                .insert(id, user)
                .ok_or(Error::new("Insert was failed".to_string()))
        }
    }
}

impl Users {
    pub fn get(&self, user_id: &Id) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn get_by_nick(&self, nickname: &str) -> Option<&User> {
        for (_, user) in self.users.iter() {
            if user.nickname == nickname {
                return Some(user);
            }
        }
        return None;
    }

    pub fn a(&self) -> i32 {
      1
    }
}
