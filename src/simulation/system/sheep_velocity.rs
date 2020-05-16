use crate::simulation::component::{Heading, SheepBehavior, SheepBehaviorState, Velocity};
use nalgebra::Vector2;
use specs::prelude::*;

pub struct SheepVelocitySystem;

impl<'a> System<'a> for SheepVelocitySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, SheepBehaviorState>,
        ReadStorage<'a, Heading>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (behavior_storage, heading_storage, mut velocity_storage) = data;

        for (behavior, heading, mut vel) in
            (&behavior_storage, &heading_storage, &mut velocity_storage).join()
        {
            vel.v = match behavior.behavior {
                SheepBehavior::Stationary { .. } => nalgebra::zero(),
                SheepBehavior::Walking => heading.r * (Vector2::x() * 0.15),
                SheepBehavior::Running => heading.r * (Vector2::x() * 1.5),
            }
        }
    }
}
