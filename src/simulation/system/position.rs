use crate::simulation::component::{Position, Velocity};
use crate::simulation::frame::{DeltaFrame, Frame};
use specs::prelude::*;

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, DeltaFrame>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (df, vel_storage, mut pos_storage) = data;

        for (vel, mut pos) in (&vel_storage, &mut pos_storage).join() {
            let delta_secs = (df.delta * Frame::DURATION_MILLIS) as f32 / 1000.0;
            let delta_vec = vel.v * delta_secs; //  pos.v + vel.v;
            let new_pos = pos.v + delta_vec;
            // TODO: Don't use magic values.
            if new_pos.x > 0.0 && new_pos.x < 80.0 && new_pos.y > 0.0 && new_pos.y < 80.0 {
                pos.v = new_pos;
            }
        }
    }
}
