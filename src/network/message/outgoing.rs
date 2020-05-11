use crate::network::error::NetworkError;
use serde::Serialize;
use std::{convert::TryFrom, net::SocketAddr};
use tungstenite::protocol::Message;

/// Message sent by the simulation server.
#[derive(Serialize, Debug)]
pub struct OutgoingMessage {
    pub recipient: SocketAddr,
    pub agent_states: Vec<AgentState>,
}

#[derive(Serialize, Debug)]
pub struct AgentState {
    pub position: Option<(f32, f32)>,
    pub heading: Option<f32>,
}

impl OutgoingMessage {
    pub fn new(recipient: SocketAddr) -> OutgoingMessage {
        OutgoingMessage {
            recipient,
            agent_states: vec![],
        }
    }

    pub fn with_agent_state(&mut self, x: f32, y: f32, heading: f32) -> &mut OutgoingMessage {
        self.agent_states.push(AgentState {
            position: Some((x, y)),
            heading: Some(heading),
        });
        self
    }
}

impl TryFrom<OutgoingMessage> for Message {
    type Error = NetworkError;

    fn try_from(val: OutgoingMessage) -> Result<Self, Self::Error> {
        let text = serde_json::to_string(&val)?;
        Ok(Message::text(text))
    }
}
