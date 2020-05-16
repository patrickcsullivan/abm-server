use crate::simulation::{
    component::{Position, SheepBehavior, SheepBehaviorState},
    frame::{DeltaFrame, Frame},
    grid::{CellBlock, Grid},
    snapshot::{RunningSheepSnapshot, StationarySheepSnapshot, WalkingSheepSnapshot},
};
use nalgebra::Vector2;
use rand::prelude::*;
use specs::prelude::*;

pub struct SheepBehaviorSystem;

impl<'a> System<'a> for SheepBehaviorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, DeltaFrame>,
        ReadExpect<'a, CellBlock<RunningSheepSnapshot>>,
        ReadExpect<'a, CellBlock<StationarySheepSnapshot>>,
        ReadExpect<'a, CellBlock<WalkingSheepSnapshot>>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, SheepBehaviorState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            df,
            running_snapshots,
            stationary_snapshots,
            walking_snapshots,
            pos_storage,
            mut behavior_storage,
        ) = data;

        for (pos, mut behavior) in (&pos_storage, &mut behavior_storage).join() {
            // WARNING: This will panic if a frame takes more than 65535 ms.
            let delta_millis = (df.delta * Frame::DURATION_MILLIS) as u16;
            if delta_millis >= behavior.next_check_millis {
                behavior.behavior = match behavior.behavior {
                    SheepBehavior::Stationary { .. } => {
                        if is_to_running() {
                            SheepBehavior::Running
                        } else if is_stationary_to_walking(pos.v, &*walking_snapshots) {
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
                        } else if is_walking_to_stationary(pos.v, &*stationary_snapshots) {
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

fn is_stationary_to_walking(
    pos: Vector2<f32>,
    walking_snapshots: &CellBlock<WalkingSheepSnapshot>,
) -> bool {
    // TODO: Factor out grid_pos calculation.
    let grid_pos = (pos.x as usize % 5, pos.y as usize % 5);
    let cell = walking_snapshots.at(grid_pos);

    // Estimate the number of walking sheep within 1 meter.
    let walking_metric_neighbor_count = match cell {
        Some(WalkingSheepSnapshot { count }) => {
            *count as f32 / 7.9618 // 7.9618 = 25 m^2 / PI m^2
        }
        _ => 0.0,
    };

    // Calculate probability of transitioning.
    const SPONTANEOUS_TRANS_TIME: f32 = 35.0; // seconds
    let p = (1.0 + MIMETIC_EFFECT * walking_metric_neighbor_count) / SPONTANEOUS_TRANS_TIME;

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < p
}

fn is_walking_to_stationary(
    pos: Vector2<f32>,
    stationary_snapshots: &CellBlock<StationarySheepSnapshot>,
) -> bool {
    // TODO: Factor out grid_pos calculation.
    let grid_pos = (pos.x as usize % 5, pos.y as usize % 5);
    let cell = stationary_snapshots.at(grid_pos);

    // Estimate the number of stationary sheep within 1 meter.
    let stationary_metric_neighbor_count = match cell {
        Some(StationarySheepSnapshot { count }) => {
            *count as f32 / 7.9618 // 7.9618 = 25 m^2 / PI m^2
        }
        _ => 0.0,
    };

    // Calculate probability of transitioning.
    const SPONTANEOUS_TRANS_TIME: f32 = 8.0; // seconds
    let p = (1.0 + MIMETIC_EFFECT * stationary_metric_neighbor_count) / SPONTANEOUS_TRANS_TIME;

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < p
}

fn is_to_running() -> bool {
    const SPONTANEOUS_TRANS_TIME: f32 = 25.0; // seconds
    false
}

fn is_running_to_stationary() -> bool {
    const SPONTANEOUS_TRANS_TIME: f32 = 25.0; // seconds
    false
}
