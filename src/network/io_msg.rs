use super::super::channel;
use super::super::geometry::BoundingBox;
use super::error::ClientError;
use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use tungstenite::protocol::Message;

/// `FromClient` is a message is sent from a client to a simulation server.
#[derive(Deserialize, Debug)]
pub enum FromClient {
    RegisterInterest(BoundingBox),
}

/// `ToClient` is a message that is sent to a client from a simulation server.
#[derive(Serialize, Debug)]
pub struct ToClient {
    cell_updates: Vec<CellUpdate>,
}

#[derive(Serialize, Debug)]
struct CellUpdate {
    x: i32,
    y: i32,
    grass: i32,
}

impl TryFrom<Message> for FromClient {
    type Error = ClientError;

    fn try_from(ws_msg: Message) -> Result<Self, Self::Error> {
        let text = ws_msg.into_text()?;
        let from_client = serde_json::from_str(&text)?;
        Ok(from_client)
    }
}

impl TryFrom<ToClient> for Message {
    type Error = ClientError;

    fn try_from(val: ToClient) -> Result<Self, Self::Error> {
        let text = serde_json::to_string(&val)?;
        Ok(Message::text(text))
    }
}

impl From<channel::ClientHandlerMsg> for ToClient {
    fn from(msg: channel::ClientHandlerMsg) -> Self {
        ToClient {
            cell_updates: msg
                .cell_updates
                .iter()
                .map(|up| CellUpdate {
                    x: up.x,
                    y: up.y,
                    grass: up.grass,
                })
                .collect(),
        }
    }
}
