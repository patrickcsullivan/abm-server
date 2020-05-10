use super::super::channel;
use crate::network::error::NetworkError;
use serde::Serialize;
use std::convert::{From, TryFrom};
use tungstenite::protocol::Message;

/// Message sent by the simulation server.
#[derive(Serialize, Debug)]
pub struct OutgoingMessage {
    cell_updates: Vec<CellUpdate>,
}

#[derive(Serialize, Debug)]
struct CellUpdate {
    x: i32,
    y: i32,
    grass: i32,
}

impl TryFrom<OutgoingMessage> for Message {
    type Error = NetworkError;

    fn try_from(val: OutgoingMessage) -> Result<Self, Self::Error> {
        let text = serde_json::to_string(&val)?;
        Ok(Message::text(text))
    }
}

impl From<channel::ClientHandlerMsg> for OutgoingMessage {
    fn from(msg: channel::ClientHandlerMsg) -> Self {
        OutgoingMessage {
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
