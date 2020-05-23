use crate::simulation::{
    component::{Position, SheepBehavior, SheepBehaviorState},
    entity_rtree::{EntityPosition, EntityRTree, IntoNaturalNeighborIterator},
    frame::{DeltaFrame, Frame},
};
use nalgebra::Vector2;
use rand::prelude::*;
use specs::prelude::*;

pub struct SheepBehaviorSystem;

impl<'a> System<'a> for SheepBehaviorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, DeltaFrame>,
        ReadExpect<'a, EntityRTree>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, SheepBehaviorState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (df, rtree, entities, pos_storage, mut behavior_storage) = data;

        for (pos, entity, mut behavior) in (&pos_storage, &entities, &mut behavior_storage).join() {
            // WARNING: This will panic if a frame takes more than 65535 ms.
            let delta_millis = (df.delta * Frame::DURATION_MILLIS) as u16;
            if delta_millis >= behavior.next_check_millis {
                behavior.behavior = match behavior.behavior {
                    SheepBehavior::Stationary { .. } => {
                        if is_to_running(entity, pos.v, &*rtree) {
                            SheepBehavior::Running
                        } else if is_stationary_to_walking(pos.v, &*rtree) {
                            SheepBehavior::Walking
                        } else {
                            // Remain stationary.
                            SheepBehavior::Stationary {
                                was_running_last_update: false,
                            }
                        }
                    }
                    SheepBehavior::Walking => {
                        if is_to_running(entity, pos.v, &*rtree) {
                            SheepBehavior::Running
                        } else if is_walking_to_stationary(pos.v, &*rtree) {
                            SheepBehavior::Stationary {
                                was_running_last_update: false,
                            }
                        } else {
                            // Keep walking.
                            behavior.behavior
                        }
                    }
                    SheepBehavior::Running => {
                        if is_running_to_stationary(entity, pos.v, &*rtree) {
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

/// A constant paramter that affects the likely of a sheep to transition to the
/// same behavior state as nearby sheep.
const BEHAVIOR_MIMETIC_EFFECT: f32 = 15.0;

/// The distance in meters within which non-running sheep will influence each
/// other's behavior. Represented as r0 in the literature.
const METRIC_INTERACTION_RANGE: f32 = 1.0;

/// A constant unitless parameter that is applied as an exponent to the mimetic
/// and grouping terms when calculating the probability that a sheep will start
/// running or stop running.
const BEHAVIOR_MIMETIC_EXP: i32 = 4;

/// A constant paramter in meters that is compared to the average distance of
/// natural natural neighbors when determining if a sheep starts running or
/// stops running. When the average natural neighbor distance is high compared
/// to the characteristic length scale a sheep will be more likely to start
/// running. When the average natural neighbor distance is low compared to the
/// characteristic length scale a sheep will be more likely to stop running.
const CHARACTERISTIC_LEN_SCALE: f32 = 36.0;

fn is_stationary_to_walking(pos: Vector2<f32>, rtree: &EntityRTree) -> bool {
    // Calculate the number of walking sheep within 1 meter.
    let walking_metric_neighbors: Vec<&EntityPosition> = rtree
        .lookup_in_circle(&[pos.x, pos.y], &METRIC_INTERACTION_RANGE)
        .into_iter()
        .filter(|epos| epos.behavior.behavior == SheepBehavior::Walking)
        .collect();

    // Calculate probability of transitioning.
    const SPONTANEOUS_TRANS_TIME: f32 = 35.0; // seconds
    let p = (1.0 + BEHAVIOR_MIMETIC_EFFECT * walking_metric_neighbors.len() as f32)
        / SPONTANEOUS_TRANS_TIME;

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < p
}

fn is_walking_to_stationary(pos: Vector2<f32>, rtree: &EntityRTree) -> bool {
    // Calculate the number of walking sheep within 1 meter.
    let stationary_metric_neighbors: Vec<&EntityPosition> = rtree
        .lookup_in_circle(&[pos.x, pos.y], &METRIC_INTERACTION_RANGE)
        .into_iter()
        .filter(|epos| {
            if let SheepBehavior::Stationary { .. } = epos.behavior.behavior {
                true
            } else {
                false
            }
        })
        .collect();

    // Calculate probability of transitioning.
    const SPONTANEOUS_TRANS_TIME: f32 = 8.0; // seconds
    let p = (1.0 + BEHAVIOR_MIMETIC_EFFECT * stationary_metric_neighbors.len() as f32)
        / SPONTANEOUS_TRANS_TIME;

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < p
}

fn is_to_running(entity: Entity, pos: Vector2<f32>, rtree: &EntityRTree) -> bool {
    let natural_neighbors: Vec<&EntityPosition> = rtree
        .natural_neighbor_iterator(&[pos.x, pos.y], |epos| epos.entity != entity)
        .take(4)
        .collect();
    let mean_dist = natural_neighbors
        .iter()
        .map(|&epos| {
            let [ex, ey] = epos.position;
            let dx = ex - pos.x;
            let dy = ey - pos.y;
            (dx * dx + dy * dy).sqrt()
        })
        .sum::<f32>()
        / natural_neighbors.len() as f32;
    let running_natural_neighbors: Vec<&EntityPosition> = natural_neighbors
        .into_iter()
        .filter(|epos| epos.behavior.behavior == SheepBehavior::Running)
        .collect();

    // Calculate probability of transitioning.
    const SPONTANEOUS_TRANS_TIME: f32 = 25.0; // N seconds, where N = number of sheep
    let trans_time_factor = 1.0 / SPONTANEOUS_TRANS_TIME;
    let separation_factor = mean_dist / CHARACTERISTIC_LEN_SCALE;
    let running_neighbors_factor =
        1.0 + BEHAVIOR_MIMETIC_EFFECT * running_natural_neighbors.len() as f32;
    let p = trans_time_factor
        * (separation_factor * running_neighbors_factor).powi(BEHAVIOR_MIMETIC_EXP);

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < p
}

fn is_running_to_stationary(entity: Entity, pos: Vector2<f32>, rtree: &EntityRTree) -> bool {
    let natural_neighbors: Vec<&EntityPosition> = rtree
        .natural_neighbor_iterator(&[pos.x, pos.y], |epos| epos.entity != entity)
        .take(4)
        .collect();
    let mean_dist = natural_neighbors
        .iter()
        .map(|&epos| {
            let [ex, ey] = epos.position;
            let dx = ex - pos.x;
            let dy = ey - pos.y;
            (dx * dx + dy * dy).sqrt()
        })
        .sum::<f32>()
        / natural_neighbors.len() as f32;
    let stopping_natural_neighbors: Vec<&EntityPosition> = natural_neighbors
        .into_iter()
        .filter(|epos| {
            if let SheepBehavior::Stationary {
                was_running_last_update: true,
            } = epos.behavior.behavior
            {
                true
            } else {
                false
            }
        })
        .collect();

    // Calculate probability of transitioning.
    const SPONTANEOUS_TRANS_TIME: f32 = 25.0; // N seconds, where N = number of sheep
    let trans_time_factor = 1.0 / SPONTANEOUS_TRANS_TIME;
    let proximity_factor = CHARACTERISTIC_LEN_SCALE / mean_dist;
    let running_neighbors_factor =
        1.0 + BEHAVIOR_MIMETIC_EFFECT * stopping_natural_neighbors.len() as f32;
    let p = trans_time_factor
        * (proximity_factor * running_neighbors_factor).powi(BEHAVIOR_MIMETIC_EXP);

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < p
}
