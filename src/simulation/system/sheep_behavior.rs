use crate::simulation::{
    component::{Position, SheepBehavior, SheepBehaviorState},
    frame::{DeltaFrame, Frame},
    grid::CellBlock,
    snapshot::RunningSheepSnapshot,
};
use rand::distributions::{Distribution, Uniform};
use specs::prelude::*;

pub struct SheepBehaviorSystem;

impl<'a> System<'a> for SheepBehaviorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, DeltaFrame>,
        ReadExpect<'a, CellBlock<RunningSheepSnapshot>>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, SheepBehaviorState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (df, running_snapshots, pos_storage, mut behavior_storage) = data;

        for (pos, mut behavior) in (&pos_storage, &mut behavior_storage).join() {
            // WARNING: This will panic if a frame takes more than 65535 ms.
            let delta_millis = (df.delta * Frame::DURATION_MILLIS) as u16;
            if delta_millis >= behavior.next_check_millis {
                behavior.behavior = match behavior.behavior {
                    SheepBehavior::Stationary { .. } => {
                        if is_to_running() {
                            SheepBehavior::Running
                        } else if is_stationary_to_walking() {
                            SheepBehavior::Walking
                        } else {
                            // Remain stationary.
                            SheepBehavior::Stationary {
                                was_running_last_update: false,
                            }
                        }
                    }
                    SheepBehavior::Walking => {
                        if is_to_running() {
                            SheepBehavior::Running
                        } else if is_walking_to_stationary() {
                            SheepBehavior::Stationary {
                                was_running_last_update: false,
                            }
                        } else {
                            // Keep walking.
                            behavior.behavior
                        }
                    }
                    SheepBehavior::Running => {
                        if is_running_to_stationary() {
                            SheepBehavior::Stationary {
                                was_running_last_update: true,
                            }
                        } else {
                            // Keep running.
                            behavior.behavior
                        }
                    }
                };
                behavior.next_check_millis = SheepBehaviorState::CHECK_PERIOD_MILLIS;
            } else {
                behavior.next_check_millis -= delta_millis;
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
