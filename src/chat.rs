mod messages;
mod store;

use crate::chat::store::users::User;
use crate::id_record::{self, IDControl};
use futures::channel::mpsc::unbounded;
use futures_util::{future, stream::TryStreamExt, StreamExt};
use messages::{
    requests, responses,
    Msg::{AuthRegister, CreateDialog, Unexpected, UserMsg},
};
use serde_json::{json, Value};
use std::{
    env,
    sync::{Arc, Mutex},
};
use store::{selectors, users, PeerItem, Store};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

type SharedStore = Arc<Mutex<Store>>;

fn response(peer: &PeerItem, msg: &Value) {
    peer.tx
        .unbounded_send(Message::Text(msg.to_string()))
        .unwrap();
}

fn response_error(peer: &PeerItem, err: responses::error::Error) {
    let packet = responses::Packet {
        action_type: responses::ActionType::Error,
        payload: responses::Payload::Error(err),
    };
    response(peer, &serde_json::to_value(packet).unwrap());
}

#[inline(always)]
fn with_auth_mw<F: FnOnce(&User) -> ()>(
    store: &Store,
    peer: &PeerItem,
    f: F,
    err: responses::error::Error,
) {
    let maybe_user = peer.user_id.and_then(|user_id| store.users.get(&user_id));

    match maybe_user {
        Some(user) => {
            f(&user);
        }
        None => {
            response_error(peer, err);
        }
    }
}

#[inline(always)]
fn with_auth<F: FnOnce(&User) -> ()>(store: &Store, peer: &PeerItem, f: F) {
    with_auth_mw(
        store,
        peer,
        f,
        responses::error::forbidden("This action require autorize"),
    )
}

#[inline(always)]
fn with_auth_companion<F: FnOnce(&User) -> ()>(store: &Store, peer: &PeerItem, f: F) {
    with_auth_mw(
        store,
        peer,
        f,
        responses::error::not_found("Your companion probably is offline"),
    )
}

#[inline]
fn handle_usr_msg(store: &Store, msg: &requests::UserMsg, sender_peer: &PeerItem, sender: &User) {
    match selectors::user_and_peer(store, &msg.receiver_id) {
        Some((receiver_peer, _)) => {
            let response_msg = json!(msg.to_response(sender.id));
            response(receiver_peer, &response_msg);
            response(sender_peer, &response_msg);
        }
        None => {
            let error_msg = format!("Cannot find user with id: {}", msg.receiver_id);
            response(
                sender_peer,
                &json!(messages::responses::error::not_found(&error_msg)),
            );
            println!(
                "[TODO: RESPONSE] User was not authorize {}",
                msg.receiver_id
            );
        }
    }
}

#[inline]
fn handle_register(store: SharedStore, msg: &requests::auth::Register, sender_peer: &PeerItem) {
    let usr = store.lock().unwrap().users.add_user(
        msg.name.clone(),
        msg.nickname.clone(),
        msg.password.clone(),
    );

    match usr {
        Ok(user) => {
            let packet = responses::Packet {
                action_type: responses::ActionType::Registered,
                payload: responses::Payload::Registered(responses::registered::Registered { user }),
            };
            response(sender_peer, &serde_json::to_value(packet).unwrap());
        }
        Err(error) => response_error(sender_peer, responses::error::bad_request(&error.message)),
    }
}

#[inline]
fn xxx(store: SharedStore, user: &User, sender_peer: &PeerItem) {
    match store.clone().lock().unwrap().dialogs.create_direct(user.id) {
        Ok(dialog) => {
            let created = responses::CreatedDialog { dialog };
            let payload = responses::Payload::CreatedDialog(created);
            let packet = responses::Packet {
                action_type: responses::ActionType::Registered,
                payload: payload.clone(),
            };
            response(sender_peer, &serde_json::to_value(packet).unwrap());
        }
        Err(_) => response_error(
            sender_peer,
            responses::error::bad_request("Can't create dialog... :("),
        ),
    }
}

#[inline]
fn handle_create_dialog(store: SharedStore, msg: &requests::CreateDialog, sender_peer: &PeerItem) {
    let usr = store.lock().unwrap().users.get_by_nick(&msg.nickname);

    match usr {
        Some(user) => xxx(store.clone(), &user, sender_peer),
        None => response_error(
            sender_peer,
            responses::error::bad_request("Can't find user with such nickname"),
        ),
    }
}

#[inline]
fn handle_parse_msg(store: SharedStore, message: &Message, sender_peer: &PeerItem) {
    let parsed_msg = messages::parse(message);
    match parsed_msg {
        UserMsg(msg) => {
            let store = &store.lock().unwrap();
            with_auth(store, sender_peer, |sender| {
                handle_usr_msg(store, &msg, &sender_peer, sender)
            })
        }
        CreateDialog(msg) => with_auth(&store.lock().unwrap(), sender_peer, |_| {
            handle_create_dialog(store.clone(), &msg, &sender_peer)
        }),
        AuthRegister(payload) => handle_register(store.clone(), &payload, &sender_peer),
        Unexpected(error) => {
            println!("Invalid message: {}", error.message);
        }
    }
}

#[inline(always)]
async fn handle_message(store: SharedStore, message: Message, sender_id: id_record::KeyType) {
    let sender_peer = store.lock().unwrap().peers.get_item_value(&sender_id);

    match sender_peer {
        Some(sender_peer) => handle_parse_msg(store.clone(), &message, &sender_peer),
        None => {
            println!("Invalid sender id: {}", sender_id);
        }
    }
}

#[inline]
async fn handle_stream(store: SharedStore, stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    let peer_item = PeerItem::new(addr, tx);
    let id = store.lock().unwrap().peers.save(peer_item);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each_concurrent(8, |msg| async {
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
