pub mod dialogs;
pub mod selectors;
pub mod users;

use crate::id_record::IDRecord;
pub use dialogs::{Dialog, Dialogs};
use futures::channel::mpsc::UnboundedSender;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::Message;
pub use users::{User, Users};

pub type Tx = UnboundedSender<Message>;

#[derive(Clone)]
pub struct PeerItem {
    pub tx: Tx,
    pub socket_addr: SocketAddr,
    pub user_id: Option<users::Id>,
}

impl PeerItem {
    pub fn new(socket_addr: SocketAddr, tx: Tx) -> Self {
        PeerItem {
            tx,
            socket_addr,
            user_id: None,
        }
    }
}

pub type Peers = IDRecord<PeerItem>;

pub struct Store {
    pub users: Users,
    pub peers: Peers,
    pub dialogs: Dialogs,
}

impl Store {
    pub fn new() -> Self {
        Store {
            users: Users::new(),
            peers: Peers::new(),
            dialogs: Dialogs::new(),
        }
    }
}

pub fn new() -> Store {
    Store::new()
}
