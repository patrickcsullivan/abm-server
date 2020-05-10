use super::geometry::BoundingBox;

use std::{collections::HashMap, net::SocketAddr};

use futures_channel::mpsc::UnboundedSender;

pub struct Manager {
    sim: Option<UnboundedSender<SimMsg>>,
    client_handlers: HashMap<SocketAddr, UnboundedSender<ClientHandlerMsg>>,
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

impl Manager {
    pub fn new() -> Manager {
        Manager {
            sim: None,
            client_handlers: HashMap::new(),
        }
    }

    pub fn insert_client_handler(
        &mut self,
        addr: SocketAddr,
        sender: UnboundedSender<ClientHandlerMsg>,
    ) {
        self.client_handlers.insert(addr, sender);
    }

    pub fn insert_sim(&mut self, sender: UnboundedSender<SimMsg>) {
        self.sim = Some(sender);
    }

    pub fn remove_client_handler(&mut self, addr: &SocketAddr) {
        self.client_handlers.remove(&addr);
    }

    /// Attempts to send a message to the simulation's channel.
    pub fn send_to_sim(&self, msg: SimMsg) {
        if let Some(sender) = &self.sim {
            let _ = sender.unbounded_send(msg);
        }
    }

    /// Attempts to send a message to a client handler's channel.
    pub fn send_to_client_handler(&self, addr: &SocketAddr, msg: ClientHandlerMsg) {
        if let Some(sender) = &self.client_handlers.get(addr) {
            let _ = sender.unbounded_send(msg);
        }
    }
}
