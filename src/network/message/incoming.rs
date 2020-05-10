use crate::network::error::NetworkError;
use serde::Deserialize;
use std::convert::TryFrom;
use tungstenite::protocol::Message;

/// Message sent to the server.
#[derive(Deserialize, Debug)]
pub struct IncomingMessage();

impl TryFrom<Message> for IncomingMessage {
    type Error = NetworkError;

    fn try_from(ws_msg: Message) -> Result<Self, Self::Error> {
        // let text = ws_msg.into_text()?;
        // let from_client = serde_json::from_str(&text)?;
        // Ok(from_client)
        Ok(IncomingMessage())
    }
}
