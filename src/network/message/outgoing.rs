use crate::network::error::NetworkError;
use serde::Serialize;
use std::{convert::TryFrom, net::SocketAddr};
use tungstenite::protocol::Message;

/// Message sent by the simulation server.
#[derive(Serialize, Debug)]
pub struct OutgoingMessage {
    pub recipient: SocketAddr,
    pub cell_updates: Vec<CellUpdate>,
}

#[derive(Serialize, Debug)]
pub struct CellUpdate {
    pub x: i32,
    pub y: i32,
    pub grass: i32,
}

impl OutgoingMessage {
    pub fn new(recipient: SocketAddr, cell_updates: Vec<CellUpdate>) -> OutgoingMessage {
        OutgoingMessage {
            recipient,
            cell_updates,
        }
    }
}

impl TryFrom<OutgoingMessage> for Message {
    type Error = NetworkError;

    fn try_from(val: OutgoingMessage) -> Result<Self, Self::Error> {
        let text = serde_json::to_string(&val)?;
        Ok(Message::text(text))
    }
}
