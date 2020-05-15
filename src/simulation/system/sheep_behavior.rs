use crate::simulation::component::{Heading, Position, SheepBehavior, SheepBehaviorState};
use crate::simulation::grid::CellBlock;
use crate::simulation::snapshot::RunningSheepSnapshot;
use nalgebra::{Rotation2, Vector2};
use rand::distributions::{Distribution, Uniform};
use specs::prelude::*;

pub struct SheepBehaviorSystem;

impl<'a> System<'a> for SheepBehaviorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, CellBlock<RunningSheepSnapshot>>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, SheepBehaviorState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (running_snapshots, pos_storage, mut behavior_storage) = data;

        for (pos, mut behavior) in (&pos_storage, &mut behavior_storage).join() {
            match behavior.behavior {
                SheepBehavior::Stationary => {}
                SheepBehavior::Walking => {}
                SheepBehavior::Running => {}
            }
        }
    }
}

const MIMETIC_EFFECT: f32 = 15.0;

fn is_stationary_to_walking() -> bool {
    const SPONTANEOUS_TRANS_TIME: f32 = 35.0; // seconds
    false
}

fn is_walking_to_stationary() -> bool {
    const SPONTANEOUS_TRANS_TIME: f32 = 8.0; // seconds
    false
}

fn is_to_running() -> bool {
    const SPONTANEOUS_TRANS_TIME: f32 = 25.0; // seconds
    false
}

fn is_running_to_stationary() -> bool {
    const SPONTANEOUS_TRANS_TIME: f32 = 25.0; // seconds
    false
}
