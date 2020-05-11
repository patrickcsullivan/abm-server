use crate::network;
use crate::simulation::component::{Heading, Position, Socket};
use specs::prelude::*;

pub struct OutboxSystem;

impl<'a> System<'a> for OutboxSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Vec<network::OutgoingMessage>>,
        Entities<'a>,
        ReadStorage<'a, Socket>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut outbox, entities, socket_storage, pos_storage, heading_storage) = data;

        // FIXME: This will be inefficent with >1 client since we'll loop
        // through all entities for each client. See below for better solution.
        for socket in socket_storage.join() {
            let mut msg = network::OutgoingMessage::new(socket.addr);
            for (pos, heading) in (&pos_storage, &heading_storage).join() {
                msg.with_agent_state(pos.v.x, pos.v.y, heading.r.angle());
            }
            outbox.push(msg);
        }

        // let query = (
        //     (&pos_storage).maybe(),
        //     (&heading_storage).maybe(),
        //     (&sheep_behavior_storage).maybe(),
        // );
        // for (pos, heading, behavior) in query.join() {
        //     // TODO: Loop through sockets with registered interests. If the
        //     // entity matches its interest add a message to the outbox. Add
        //     // logic to the outbox so that a new message added to the same
        //     // sender are packed and sent together as a single message.
        // }
    }
}
