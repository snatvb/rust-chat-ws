extern crate tokio;

use futures::future::poll_fn;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    for i in (0..9999999) {
      let x = (i as i64).wrapping_mul(i);
      println!("{}", x);
    }
    let ws_stream: WebSocketStream<TcpStream> = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    // let stream = ws_stream.poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>);
    // let mut buf = [0; 10];

    // poll_fn(|cx| {
    //     stream.poll_peek(cx, &mut buf)
    // }).await.unwrap();
    // let msg = ws_stream.inner.read_message().unwrap();

    // read.forward(write)
    //     .await
    //     .expect("Failed to forward message")
}

async fn run_server() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8089".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async || {
          for i in 0..999999 {
            let x = (i as i64).wrapping_mul(i);
            println!("{}", x);
          }
        });
        // tokio::spawn(accept_connection(stream));
    }
}

#[tokio::main]
async fn main() {
    println!("Start web socket server");
    let server = run_server();
    server.await;
}

// let mut rng = rand::thread_rng();
// let sleep_duration = rng.gen_range(0, 50);
// println!("Sleep: {}", sleep_duration);
// thread::sleep_ms(sleep_duration);
