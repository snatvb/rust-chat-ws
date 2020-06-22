pub mod requests;
pub mod responses;

use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;


#[derive(PartialEq, Clone)]
pub struct Error {
  pub message: String,
}

pub enum Msg {
  AuthRegister(requests::auth::Register),
  UserMsg(requests::UserMsg),
  Unexpected(Error),
}

#[inline(always)]
fn error(string: &str) -> Error {
  Error {
    message: string.to_string(),
  }
}

#[inline]
fn parse_usr_message(json_payload: Value) -> Msg {
  let parsed: Option<requests::UserMsg> = serde_json::from_value(json_payload).ok();
  parsed.map(Msg::UserMsg).unwrap_or(Msg::Unexpected(error("Message parse was failed")))
}

#[inline]
fn parse_register(json_payload: Value) -> Msg {
  let parsed: Option<requests::auth::Register> = serde_json::from_value(json_payload).ok();
  parsed.map(Msg::AuthRegister).unwrap_or(Msg::Unexpected(error("Register payload parse was failed")))
}

#[inline]
fn parse_payload(action_type: &str, json_payload: Value) -> Msg {
  match action_type {
    "Message" => parse_usr_message(json_payload),
    "Register" => parse_register(json_payload),
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
