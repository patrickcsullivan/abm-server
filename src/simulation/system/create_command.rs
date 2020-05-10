use crate::simulation::command_queue::CreateSheepCommandQueue;
use crate::simulation::component::{Heading, Position, SheepBehaviorState, Velocity};
use specs::prelude::*;

pub struct CreateCommandSystem;

impl<'a> System<'a> for CreateCommandSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, CreateSheepCommandQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Heading>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, SheepBehaviorState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut command_queue,
            entities,
            mut pos_storage,
            mut heading_storage,
            mut vel_storage,
            mut behavior_storage,
        ) = data;

        for cmd in command_queue.commands.iter() {
            let e = entities.create();
            pos_storage
                .insert(e, cmd.position)
                .expect("Unable to insert position.");
            heading_storage
                .insert(e, cmd.heading)
                .expect("Unable to insert heading.");
            vel_storage
                .insert(e, cmd.velocity)
                .expect("Unable to insert velocity.");
            behavior_storage
                .insert(e, cmd.behavior)
                .expect("Unable to insert behavior.");
        }

        command_queue.clear();
    }
}
