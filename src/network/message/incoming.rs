use crate::network::error::NetworkResult;
use serde::Deserialize;
use std::net::SocketAddr;
use tungstenite::protocol::Message;

/// Message sent to the server.
#[derive(Deserialize, Debug)]
pub struct IncomingMessage {
    pub sender: SocketAddr,
}

impl IncomingMessage {
    pub fn try_new(sender: SocketAddr, _ws_msg: Message) -> NetworkResult<IncomingMessage> {
        Ok(IncomingMessage { sender })
    }
}
