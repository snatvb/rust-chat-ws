use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

pub mod types;

#[derive(PartialEq, Clone)]
pub struct Error {
  pub message: String,
}

pub enum Msg {
  UserMsg(types::UserMsg),
  Unexpected(Error),
}

fn error(string: &str) -> Error {
  Error {
    message: string.to_string(),
  }
}

fn parse_usr_message(json_payload: Value) -> Msg {
  let user_msg: Option<types::UserMsg> = serde_json::from_value(json_payload).ok();
  match user_msg.map(Msg::UserMsg) {
    Some(x) => x,
    None => Msg::Unexpected(error("Message parse was failed")),
  }
}

fn parse_payload(action_type: &str, json_payload: Value) -> Msg {
  match action_type {
    "message" => parse_usr_message(json_payload),
    _ => Msg::Unexpected(error(&format!("Unexpected action type {}", "action_type"))),
  }
}

pub fn parse(message: &Message) -> Msg {
    let json_msg: Value = message
        .to_text()
        .ok()
        .and_then(|x| serde_json::from_str(x).ok())
        .unwrap_or_default();

    match &json_msg["type"] {
        Value::String(action_type) => {
            parse_payload(action_type, json_msg["payload"].clone())
        }
        _ => {
            Msg::Unexpected(error(&format!("Parse failed: {:?}", &json_msg)))
        }
    }
}
