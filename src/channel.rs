use super::geometry::BoundingBox;

use std::{collections::HashMap, net::SocketAddr};

use futures_channel::mpsc::UnboundedSender;

/// Contains the sender end of a channel for messages to the simulation and the
/// sender ends of channels for messages to connected clients.
pub struct SenderManager {
    /// The sender end of a channel that the server will listen to for incoming
    /// messages. Messages sent on this channel will be sent to the simulation.
    sim_sender: Option<UnboundedSender<SimMsg>>,

    /// Contains the sender end of a channel for each client connected to the
    /// server. Messages sent on one of these channels will be forwarded to the
    /// connection on the respective socket.
    client_senders: HashMap<SocketAddr, UnboundedSender<ClientHandlerMsg>>,
}

pub enum SimMsg {
    RegisterInterest(SocketAddr, BoundingBox),
}

pub struct ClientHandlerMsg {
    pub cell_updates: Vec<CellUpdate>,
}

pub struct CellUpdate {
    pub x: i32,
    pub y: i32,
    pub grass: i32,
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
        sender: UnboundedSender<ClientHandlerMsg>,
    ) {
        self.client_senders.insert(addr, sender);
    }

    pub fn insert_sim_sender(&mut self, sender: UnboundedSender<SimMsg>) {
        self.sim_sender = Some(sender);
    }

    pub fn remove_client_sender(&mut self, addr: &SocketAddr) {
        self.client_senders.remove(&addr);
    }

    /// Attempts to send a message on the simulation's channel.
    pub fn send_to_sim(&self, msg: SimMsg) {
        if let Some(sender) = &self.sim_sender {
            let _ = sender.unbounded_send(msg);
        }
    }

    /// Attempts to send a message on the client's channel channel.
    pub fn send_to_client(&self, addr: &SocketAddr, msg: ClientHandlerMsg) {
        if let Some(sender) = &self.client_senders.get(addr) {
            let _ = sender.unbounded_send(msg);
        }
    }
}
