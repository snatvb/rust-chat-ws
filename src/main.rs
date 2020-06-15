extern crate tokio;
extern crate websocket;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use websocket::server::upgrade::WsUpgrade;
use websocket::sync::Server;
use websocket::OwnedMessage;

const WORKERS: usize = 4;

type Request = WsUpgrade<std::net::TcpStream, Option<websocket::server::upgrade::sync::Buffer>>;

struct SharedState {
    sender: websocket::sender::Writer<std::net::TcpStream>,
}

fn handle_request(request: Request) {
    let protocol_name = "rust-websocket";
    if !request.protocols().contains(&protocol_name.to_string()) {
        println!("Rejected");
        request.reject().unwrap();
        return;
    }

    let client = request.use_protocol(protocol_name).accept().unwrap();
    let ip = client.peer_addr().unwrap();
    println!("Connection from {}", ip);

    let (mut receiver, sender) = client.split().unwrap();
    let shared_state = Arc::new(Mutex::new(SharedState { sender }));

    for message in receiver.incoming_messages() {
        let shared_state_thread = shared_state.clone();
        let mut rng = rand::thread_rng();
        let sleep_duration = rng.gen_range(0, 50);
        println!("Sleep: {}", sleep_duration);
        thread::sleep_ms(sleep_duration);
        let mut shared_state = shared_state_thread.lock().unwrap();
        let message = message.unwrap();
        match message {
            OwnedMessage::Close(_) => {
                let message = OwnedMessage::Close(None);
                shared_state.sender.send_message(&message).unwrap();
                println!("Client {} disconnected", ip);
                return;
            }
            OwnedMessage::Ping(ping) => {
                let message = OwnedMessage::Pong(ping);
                shared_state.sender.send_message(&message).unwrap();
            }
            _ => shared_state.sender.send_message(&message).unwrap(),
        }
    }
}

async fn run_server() {
    let server = Server::bind("127.0.0.1:8089").unwrap();
    // let pool = rayon::ThreadPoolBuilder::new()
    //     .num_threads(WORKERS)
    //     .build()
    //     .unwrap();

    for request in server.filter_map(Result::ok) {
        tokio::spawn(async move {
            handle_request(request);
        });
    }
}

#[tokio::main]
async fn main() {
    println!("Start web socket server");
    let server = run_server();
    server.await;
}
