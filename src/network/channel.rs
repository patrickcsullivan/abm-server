use super::message::{IncomingMessage, OutgoingMessage};
use futures_channel::mpsc::UnboundedSender;
use std::{collections::HashMap, net::SocketAddr};

/// Contains the sender end of a channel for messages to the simulation and the
/// sender ends of channels for messages to connected clients.
pub struct SenderManager {
    /// The sender end of a channel that the server will listen to for incoming
    /// messages. Messages sent on this channel will be sent to the simulation.
    sim_sender: Option<UnboundedSender<IncomingMessage>>,

    /// Contains the sender end of a channel for each client connected to the
    /// server. Messages sent on one of these channels will be forwarded to the
    /// connection on the respective socket.
    client_senders: HashMap<SocketAddr, UnboundedSender<OutgoingMessage>>,
}

impl SenderManager {
    pub fn new() -> SenderManager {
        SenderManager {
            sim_sender: None,
            client_senders: HashMap::new(),
        }
    }

    pub fn insert_client_sender(
        &mut self,
        addr: SocketAddr,
        sender: UnboundedSender<OutgoingMessage>,
    ) {
        self.client_senders.insert(addr, sender);
    }

    pub fn insert_sim_sender(&mut self, sender: UnboundedSender<IncomingMessage>) {
        self.sim_sender = Some(sender);
    }

    pub fn remove_client_sender(&mut self, addr: &SocketAddr) {
        self.client_senders.remove(&addr);
    }

    /// Attempts to send a message on the simulation's channel.
    pub fn send_to_sim(&self, msg: IncomingMessage) {
        if let Some(sender) = &self.sim_sender {
            let _ = sender.unbounded_send(msg);
        }
    }

    /// Attempts to send a message on the client's channel.
    pub fn send_to_client(&self, msg: OutgoingMessage) {
        if let Some(sender) = &self.client_senders.get(&msg.recipient) {
            let _ = sender.unbounded_send(msg);
        }
    }
}
