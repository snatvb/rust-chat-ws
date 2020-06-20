extern crate tokio;

mod chat;
mod id_record;

#[tokio::main]
async fn main() {
  chat::start().await;
}

// let mut rng = rand::thread_rng();
// let sleep_duration = rng.gen_range(0, 50);
// println!("Sleep: {}", sleep_duration);
// thread::sleep_ms(sleep_duration);
