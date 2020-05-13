use crate::simulation::component::Socket;
use crate::simulation::network;
use specs::prelude::*;

pub struct CreateSocketSystem;

impl<'a> System<'a> for CreateSocketSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Vec<network::IncomingMessage>>,
        Entities<'a>,
        WriteStorage<'a, Socket>,
    );

    /// Creates a socket for each sender in the inbox if the socket does not
    /// exist yet.
    fn run(&mut self, data: Self::SystemData) {
        let (inbox, entities, mut socket_storage) = data;

        for msg in &*inbox {
            if socket_storage
                .join()
                .find(|&&s| s.addr == msg.sender)
                .is_none()
            {
                let e = entities.create();
                socket_storage
                    .insert(e, Socket::new(msg.sender))
                    .expect("Unable to insert position.");
            }
        }
    }
}
