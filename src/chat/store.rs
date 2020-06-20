mod user;

use futures::channel::mpsc::{UnboundedSender};
use std::{net::SocketAddr};
use tokio_tungstenite::tungstenite::Message;
use crate::id_record::IDRecord;
pub use user::User;

pub type Tx = UnboundedSender<Message>;

pub struct PeerItem {
  pub socket_addr: SocketAddr,
  pub tx: Tx,
  pub user: Option<User>,
}

pub type Peers = IDRecord<u32, PeerItem>;

pub struct Store {
    pub peers: Peers,
}

impl Store {
    pub fn new() -> Self {
        Store {
            peers: Peers::new(),
        }
    }
}

pub fn new() -> Store {
    Store::new()
}
