use crate::id_record::IDControl;
use futures::channel::mpsc::unbounded;
use futures_util::{future, stream::TryStreamExt, StreamExt};
use serde_json::Value;
use std::{
    env,
    sync::{Arc, Mutex},
};
use store::{PeerItem, Store};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message};
use messages::Msg::{Unexpected, UserMsg};
use messages::types;
use serde_json::json;

mod store;
mod messages;

type SharedStore = Arc<Mutex<Store>>;

fn response(peer: &PeerItem, s: &Value) {
  peer.tx.unbounded_send(Message::Text(s.to_string())).unwrap();
}

fn handle_usr_msg(store: SharedStore, msg: &types::UserMsg, sender_id: u32) {
  let store = &store.lock().unwrap();
  let receiver_peer = store.peers.get(&msg.receiver_id);
  match receiver_peer {
    Some((_, receiver_peer)) => {
      let response_msg = json!(msg.to_response(sender_id));
      response(receiver_peer, &response_msg);
    },
    None => {},
  }
}

async fn handle_message(store: SharedStore, message: Message, sender_id: u32) {

    let parsed_msg = messages::parse(&message);

    match parsed_msg {
      UserMsg(msg) => handle_usr_msg(store, &msg, sender_id),
      Unexpected(error) => {
        println!("Invalid message: {}", error.message);
      },
    }
}

async fn handle_stream(store: SharedStore, stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    let peer_item = PeerItem {
        socket_addr: addr,
        tx,
        user: None,
    };
    let id = store.lock().unwrap().peers.save(peer_item);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each_concurrent(2, |msg| async {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );

        handle_message(store.clone(), msg, id).await;
        Ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    store.lock().unwrap().peers.remove(&id);
}

async fn run_server(store: &SharedStore) {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8089".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_stream(store.clone(), stream));
    }
}

pub async fn start() {
    println!("Start web socket server");
    let store = SharedStore::new(Mutex::new(store::new()));
    run_server(&store).await;
}
